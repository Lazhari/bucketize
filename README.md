# Bucketizer

> `bucketize` is a crate for slotting numerical values into buckets. To do this, create a `Bucketizer` and add you buckets to it.

> The use the `.bucketize()` method to get back the bucket a value fits into.

## Example

```rust
use bucketize::Bucketizer;
let b = Bucketizer::new()
    .bucket(Some(0.0), Some(1.0), 0.5)
    .bucket(Some(1.0), None, 1.5);
assert_eq!(b.bucketize(0.0), Some(0.5));
assert_eq!(b.bucketize(-0.7), None);
assert_eq!(b.bucketize(999.99), Some(1.5));
```

A `Bucketizer` holds the list of buckets you want slot value into, and does the bucketization operation. You can create one with `new()` and add buckets with chained `.bucket()` calls. These calls add buckets which are evaluated in order. For instance, if you add a bucket from 0 to 100 and then add a bucket from 2 to 50, nothing will ever get put in that second bucket.

Buckets are min-inclusive and mac-exclusive. If a given value matches no buckets, `bucketize` returns a `None`

## Example

```rust
use bucketize::Bucketizer;
let b = Bucketizer::new()
    .bucket(Some(10.0), Some(20.0), 15.0)
    .bucket(Some(5.0), Some(10.0), 7.5)
    .bucket(None, Some(4.0), 0.0);
assert_eq!(b.bucketize(12.34), Some(15.0));
assert_eq!(b.bucketize(9999.99), None);
```

## Create a new bucketizer

To create a new `Bucketizer` with no buckets configured.

### Example

```rust
use bucketize::Bucketizer;
let b = Bucketizer::new()
```

## Add a new bucket

Add a new bucket to the `Bucketizer`. Consumes and returns the `Bucketizer` so it can be chained.
Buckets are evaluated in the order they are added.

A value fits in a bucket if it is greater than or equal to `min` and less than `max`, if each is present

### Example

> Here, we create a `Bucketizer` with a single bucket matching any value 10

```rust
use bucketize::Bucketizer;
let b = Bucketizer::new().bucket(Some(10.0), None, 10.0);

  assert_eq!(b.bucketize(12.0), Some(10.0));
  assert_eq!(b.bucketize(-10.0), None);
```

> Here, we create a `Bucketizer` matching values from 0 to < 10 and from 10 to infinity

```rust
use bucketize::Bucketizer;
let b = Bucketizer::new()
    .bucket(Some(10.0), None, 10.0)
    .bucket(Some(0.0), Some(10.0), 5.0);

assert_eq!(b.bucketize(4.132), Some(5.0));
assert_eq!(b.bucketize(12.0), Some(10.0));
assert_eq!(b.bucketize(-20.0), None);
```
