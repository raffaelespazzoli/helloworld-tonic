package org.acme;

import java.time.Duration;
import java.time.Instant;
import java.util.List;
import java.util.Vector;
import java.util.stream.Collectors;

import helloworld.Greeter;
import helloworld.Helloworld.HelloReply;
import helloworld.Helloworld.HelloRequest;
import io.quarkus.grpc.GrpcClient;
import io.smallrye.mutiny.Uni;
import io.smallrye.mutiny.infrastructure.Infrastructure;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;

@Command
public class HelloWorldClient implements Runnable {

  @Option(names = {"-t", "--times"}, description = "how many times", defaultValue = "1")
  int times;

  @GrpcClient
  Greeter greeter;

  @Override
  public void run() {
    List<Uni<HelloReply>> listOfUni=new Vector<>(times);
    Instant start = Instant.now();
    for (int i = 0; i < times; i++ ){
      listOfUni.add(greeter.sayHello(HelloRequest.newBuilder().setName("Tonic"+i).build()));
    }
    Uni<List<HelloReply>> uniList= Uni.join().all(listOfUni).andCollectFailures();

    List<String> messages=uniList.await().indefinitely().stream().map(r -> r.getMessage()).collect(Collectors.toList());      
    Instant finish = Instant.now();
    long timeElapsed = Duration.between(start, finish).toMillis();

    messages.forEach(m -> System.out.println(m));

    System.out.println(times+" requests executed in "+timeElapsed);


  }

  Uni<String> fakeGrpcCall() {
    return task1().onItem().invoke(f -> Infrastructure.getDefaultExecutor().execute(() -> {task2(f).await().indefinitely();}));
  }

  Uni<String> task1(){
    return Uni.createFrom().item("ciao");
  }

  Uni<String> task2(String s){
    return Uni.createFrom().item(s+" raffa");
  }
}
