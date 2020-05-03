extern crate bucketize;
use bucketize::Bucketizer;

#[test]
fn multiple_buckets_open_ends() {
  let b = Bucketizer::new()
    .bucket(Some(0.0), Some(1.0), 0.5)
    .bucket(Some(1.0), None, 1.5);

  assert_eq!(b.bucketize(0.0), Some(0.5));
  assert_eq!(b.bucketize(-0.7), None);
  assert_eq!(b.bucketize(999.99), Some(1.5));
}
