Necessary components of life:
- self-replication 
- genome is replicated 
- genome can go occasional modification (1/1000 chance per bit flip)
- selection

Useful vocabulary: 
- Genome: information carried by a living entity. That dictates some thing about it

```rust 
struct LivingEntity {
  genome: Genome,
}

struct Genome {
  sequence: Vec<u8>,
}

impl Genome {
  fn copy(&self) {
    use rand;
  }
}
```

