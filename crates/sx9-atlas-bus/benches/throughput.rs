//! Throughput benchmarks for ATLAS bus

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use sx9_atlas_bus::{AtlasBus, Command, CommandKind, AtlasResult, ResultKind};

fn bench_ring_push_pop(c: &mut Criterion) {
    let bus = AtlasBus::new();
    
    // Enable SDT
    bus.plasma().prime();
    bus.plasma().trigger(0);
    
    let mut group = c.benchmark_group("ring_operations");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("push", |b| {
        b.iter(|| {
            let cmd = Command::new(CommandKind::Ping { 
                seq: black_box(1), 
                timestamp_ns: black_box(0) 
            });
            bus.dispatch(black_box(cmd));
            bus.pop(); // Keep queue from filling
        })
    });
    
    group.bench_function("pop_empty", |b| {
        b.iter(|| {
            black_box(bus.pop());
        })
    });
    
    group.finish();
}

fn bench_priority_routing(c: &mut Criterion) {
    let bus = AtlasBus::new();
    bus.plasma().prime();
    bus.plasma().trigger(0);
    
    let mut group = c.benchmark_group("priority_routing");
    group.throughput(Throughput::Elements(3));
    
    group.bench_function("dispatch_all_priorities", |b| {
        b.iter(|| {
            bus.dispatch(Command::new(CommandKind::Ping { seq: 1, timestamp_ns: 0 }));
            bus.dispatch(Command::urgent(CommandKind::Ping { seq: 2, timestamp_ns: 0 }));
            bus.dispatch(Command::critical(CommandKind::Ping { seq: 3, timestamp_ns: 0 }));
            
            // Drain
            bus.pop();
            bus.pop();
            bus.pop();
        })
    });
    
    group.finish();
}

fn bench_result_channel(c: &mut Criterion) {
    let bus = AtlasBus::new();
    
    let mut group = c.benchmark_group("result_channel");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("respond_and_pop", |b| {
        b.iter(|| {
            let result = AtlasResult::ok(
                ResultKind::MatroidRank { rank: black_box(5) },
                black_box(123)
            );
            bus.respond(black_box(result));
            bus.pop_result();
        })
    });
    
    group.finish();
}

fn bench_plasma_state(c: &mut Criterion) {
    let bus = AtlasBus::new();
    
    let mut group = c.benchmark_group("plasma_state");
    
    group.bench_function("read_delta_angle", |b| {
        b.iter(|| {
            black_box(bus.plasma().delta_angle());
        })
    });
    
    group.bench_function("write_delta_angle", |b| {
        b.iter(|| {
            bus.plasma().set_delta_angle(black_box(45.0));
        })
    });
    
    group.bench_function("sdt_trigger", |b| {
        b.iter(|| {
            bus.plasma().reset();
            bus.plasma().prime();
            bus.plasma().trigger(black_box(100));
        })
    });
    
    group.bench_function("snapshot", |b| {
        b.iter(|| {
            black_box(bus.plasma().snapshot());
        })
    });
    
    group.finish();
}

fn bench_throughput(c: &mut Criterion) {
    let bus = AtlasBus::new();
    bus.plasma().prime();
    bus.plasma().trigger(0);
    
    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Elements(1000));
    
    group.bench_function("1000_commands", |b| {
        b.iter(|| {
            for i in 0..1000u32 {
                bus.dispatch(Command::new(CommandKind::Ping { 
                    seq: i, 
                    timestamp_ns: 0 
                }));
            }
            
            for _ in 0..1000 {
                bus.pop();
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_ring_push_pop,
    bench_priority_routing,
    bench_result_channel,
    bench_plasma_state,
    bench_throughput,
);

criterion_main!(benches);




