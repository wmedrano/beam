// Licensed to the Apache Software Foundation (ASF) under one or more
// contributor license agreements. See the NOTICE file distributed with
// this work for additional information regarding copyright ownership.
// The ASF licenses this file to You under the Apache License, Version 2.0
// (the "License"); you may not use this file except in compliance with
// the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Adversarial Beam API peer. Uses the checkout's generated Go bindings and
// grpc-go, independently of Rust's protobuf runtime and RPC implementation.
package main

import (
	"context"
	"crypto/ed25519"
	"crypto/rand"
	"crypto/tls"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/json"
	"encoding/pem"
	"flag"
	"fmt"
	"io"
	"math/big"
	"net"
	"os"
	"path/filepath"
	"sync"
	"sync/atomic"
	"time"

	fnpb "github.com/apache/beam/sdks/v2/go/pkg/beam/model/fnexecution_v1"
	jobpb "github.com/apache/beam/sdks/v2/go/pkg/beam/model/jobmanagement_v1"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/structpb"
)

type peer struct {
	fnpb.UnimplementedProvisionServiceServer
	fnpb.UnimplementedBeamFnDataServer
	jobpb.UnimplementedJobServiceServer
	unblock   chan struct{}
	once      sync.Once
	cancelled atomic.Bool
}

func (p *peer) GetProvisionInfo(ctx context.Context, _ *fnpb.GetProvisionInfoRequest) (*fnpb.GetProvisionInfoResponse, error) {
	md, _ := metadata.FromIncomingContext(ctx)
	ids := md.Get("worker_id")
	if len(ids) != 1 || ids[0] != "rust-interop" {
		return nil, status.Error(codes.PermissionDenied, "worker_id mismatch")
	}
	options, _ := structpb.NewStruct(map[string]any{"parallelism": float64(42), "language": "rust"})
	return &fnpb.GetProvisionInfoResponse{Info: &fnpb.ProvisionInfo{PipelineOptions: options}}, nil
}

func (p *peer) GetState(ctx context.Context, r *jobpb.GetJobStateRequest) (*jobpb.JobStateEvent, error) {
	switch r.JobId {
	case "cancel":
		<-ctx.Done()
		p.cancelled.Store(true)
		return nil, status.FromContextError(ctx.Err()).Err()
	case "cancelled":
		if p.cancelled.Load() {
			return &jobpb.JobStateEvent{State: jobpb.JobState_CANCELLED}, nil
		}
		return &jobpb.JobStateEvent{State: jobpb.JobState_RUNNING}, nil
	case "unblock":
		p.once.Do(func() { close(p.unblock) })
		return &jobpb.JobStateEvent{State: jobpb.JobState_DONE}, nil
	default:
		return nil, status.Error(codes.NotFound, "unknown job: "+r.JobId)
	}
}

func (p *peer) GetStateStream(_ *jobpb.GetJobStateRequest, stream jobpb.JobService_GetStateStreamServer) error {
	for _, state := range []jobpb.JobState_Enum{jobpb.JobState_STARTING, jobpb.JobState_RUNNING, jobpb.JobState_DONE} {
		if err := stream.Send(&jobpb.JobStateEvent{State: state}); err != nil {
			return err
		}
	}
	return nil
}

func (p *peer) Data(stream fnpb.BeamFnData_DataServer) error {
	if err := stream.SendHeader(metadata.MD{}); err != nil {
		return err
	}
	md, _ := metadata.FromIncomingContext(stream.Context())
	if md.Get("test-backpressure") != nil {
		select {
		case <-p.unblock:
		case <-stream.Context().Done():
			return stream.Context().Err()
		}
	}
	for {
		msg, err := stream.Recv()
		if err == io.EOF {
			return nil
		}
		if err != nil {
			return err
		}
		if err := stream.Send(msg); err != nil {
			return err
		}
	}
}

func fail(err error) {
	if err != nil {
		panic(err)
	}
}

func certificates(directory string) tls.Certificate {
	public, private, err := ed25519.GenerateKey(rand.Reader)
	fail(err)
	now := time.Now()
	ca := &x509.Certificate{SerialNumber: big.NewInt(1), Subject: pkix.Name{CommonName: "Beam test CA"}, NotBefore: now.Add(-time.Hour), NotAfter: now.Add(time.Hour), IsCA: true, BasicConstraintsValid: true, KeyUsage: x509.KeyUsageCertSign}
	caDER, err := x509.CreateCertificate(rand.Reader, ca, ca, public, private)
	fail(err)
	leaf := &x509.Certificate{SerialNumber: big.NewInt(2), Subject: pkix.Name{CommonName: "localhost"}, DNSNames: []string{"localhost"}, IPAddresses: []net.IP{net.ParseIP("127.0.0.1")}, NotBefore: now.Add(-time.Hour), NotAfter: now.Add(time.Hour), KeyUsage: x509.KeyUsageDigitalSignature, ExtKeyUsage: []x509.ExtKeyUsage{x509.ExtKeyUsageServerAuth, x509.ExtKeyUsageClientAuth}}
	der, err := x509.CreateCertificate(rand.Reader, leaf, ca, public, private)
	fail(err)
	key, err := x509.MarshalPKCS8PrivateKey(private)
	fail(err)
	certPEM := pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: der})
	keyPEM := pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: key})
	fail(os.WriteFile(filepath.Join(directory, "ca.pem"), pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: caDER}), 0600))
	fail(os.WriteFile(filepath.Join(directory, "identity.pem"), certPEM, 0600))
	fail(os.WriteFile(filepath.Join(directory, "identity.key"), keyPEM, 0600))
	cert, err := tls.X509KeyPair(certPEM, keyPEM)
	fail(err)
	return cert
}

func main() {
	directory := flag.String("directory", "", "temporary directory for ephemeral TLS credentials")
	flag.Parse()
	cert := certificates(*directory)
	ca, err := os.ReadFile(filepath.Join(*directory, "ca.pem"))
	fail(err)
	roots := x509.NewCertPool()
	roots.AppendCertsFromPEM(ca)
	plain, err := net.Listen("tcp", "127.0.0.1:0")
	fail(err)
	secure, err := net.Listen("tcp", "127.0.0.1:0")
	fail(err)
	p := &peer{unblock: make(chan struct{})}
	start := func(listener net.Listener, opts ...grpc.ServerOption) {
		s := grpc.NewServer(opts...)
		fnpb.RegisterProvisionServiceServer(s, p)
		fnpb.RegisterBeamFnDataServer(s, p)
		jobpb.RegisterJobServiceServer(s, p)
		go func() { fail(s.Serve(listener)) }()
	}
	start(plain)
	start(secure, grpc.Creds(credentials.NewTLS(&tls.Config{Certificates: []tls.Certificate{cert}, ClientAuth: tls.RequireAndVerifyClientCert, ClientCAs: roots, MinVersion: tls.VersionTLS12})))
	info, _ := json.Marshal(map[string]string{"plain": plain.Addr().String(), "tls": secure.Addr().String()})
	fmt.Println(string(info))
	select {}
}
