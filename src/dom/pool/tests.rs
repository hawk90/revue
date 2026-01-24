use super::*;

#[test]
fn test_object_pool_basic() {
    let pool: ObjectPool<Vec<u8>> = ObjectPool::new(|| Vec::with_capacity(64));

    // First acquire creates new
    let mut v1 = pool.acquire();
    assert!(v1.capacity() >= 64);
    v1.push(1);
    v1.push(2);

    // Release back to pool
    pool.release(v1);
    assert_eq!(pool.size(), 1);

    // Second acquire reuses
    let v2 = pool.acquire();
    assert!(v2.capacity() >= 64);
    // Note: contents are NOT cleared by default - caller should clear if needed
    assert_eq!(pool.size(), 0);

    let stats = pool.stats();
    assert_eq!(stats.acquires, 2);
    assert_eq!(stats.misses, 1);
    assert_eq!(stats.hits, 1);
}

#[test]
fn test_object_pool_max_size() {
    let pool: ObjectPool<u32> = ObjectPool::with_capacity(|| 0, 2);

    pool.release(1);
    pool.release(2);
    pool.release(3); // Should be discarded

    assert_eq!(pool.size(), 2);
    assert_eq!(pool.stats().discards, 1);
}

#[test]
fn test_object_pool_prewarm() {
    let pool: ObjectPool<String> = ObjectPool::with_capacity(|| String::with_capacity(32), 10);
    pool.prewarm(5);

    assert_eq!(pool.size(), 5);

    // Acquiring should use pre-warmed objects
    let _ = pool.acquire();
    assert_eq!(pool.size(), 4);
}

#[test]
fn test_buffer_pool_basic() {
    let pool = BufferPool::new();

    let buf = pool.acquire(80, 24);
    assert_eq!(buf.width(), 80);
    assert_eq!(buf.height(), 24);

    pool.release(buf);
    assert_eq!(pool.total_buffered(), 1);

    // Acquire same size
    let buf2 = pool.acquire(80, 24);
    assert_eq!(buf2.width(), 80);
    assert_eq!(pool.stats().hits, 1);
}

#[test]
fn test_buffer_pool_resize() {
    let pool = BufferPool::new();

    // Release a large buffer
    let buf = Buffer::new(160, 48);
    pool.release(buf);

    // Acquire a smaller buffer - should resize the larger one
    let buf2 = pool.acquire(80, 24);
    assert_eq!(buf2.width(), 80);
    assert_eq!(buf2.height(), 24);
    assert_eq!(pool.stats().hits, 1);
}

#[test]
fn test_string_pool_basic() {
    let pool = StringPool::new();

    let s1 = pool.intern("hello");
    let s2 = pool.intern("hello");
    let s3 = pool.intern("world");

    // Same string should return same Arc
    assert!(Arc::ptr_eq(&s1, &s2));
    assert!(!Arc::ptr_eq(&s1, &s3));

    assert_eq!(pool.len(), 2);
    assert_eq!(pool.stats().hits, 1);
    assert_eq!(pool.stats().misses, 2);
}

#[test]
fn test_vec_pool_basic() {
    let pool: VecPool<i32> = VecPool::new(16);

    let mut v = pool.acquire();
    assert!(v.capacity() >= 16);
    v.push(1);
    v.push(2);

    pool.release(v);

    // Acquire should return cleared vector
    let v2 = pool.acquire();
    assert!(v2.is_empty());
    assert!(v2.capacity() >= 16);
}

#[test]
fn test_pooled_guard() {
    let pool: ObjectPool<String> = ObjectPool::new(|| String::with_capacity(32));

    {
        let mut s = Pooled::new(&pool);
        s.push_str("hello");
        assert_eq!(&*s, "hello");
    } // Dropped, returned to pool

    assert_eq!(pool.size(), 1);
}

#[test]
fn test_pooled_take() {
    let pool: ObjectPool<String> = ObjectPool::new(|| String::with_capacity(32));

    let s = {
        let mut pooled = Pooled::new(&pool);
        pooled.push_str("hello");
        pooled.take() // Take ownership
    };

    assert_eq!(s, "hello");
    assert_eq!(pool.size(), 0); // Not returned to pool
}

#[test]
fn test_sync_object_pool() {
    use std::thread;

    let pool: Arc<SyncObjectPool<Vec<u8>>> =
        Arc::new(SyncObjectPool::new(|| Vec::with_capacity(64)));

    let handles: Vec<_> = (0..4)
        .map(|_| {
            let pool = pool.clone();
            thread::spawn(move || {
                for _ in 0..10 {
                    let v = pool.acquire();
                    pool.release(v);
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let stats = pool.stats();
    assert_eq!(stats.acquires, 40);
    assert_eq!(stats.releases, 40);
}

#[test]
fn test_sync_string_pool() {
    use std::thread;

    let pool: Arc<SyncStringPool> = Arc::new(SyncStringPool::new());

    let handles: Vec<_> = (0..4)
        .map(|i| {
            let pool = pool.clone();
            thread::spawn(move || {
                for j in 0..10 {
                    let _ = pool.intern(format!("string-{}-{}", i, j));
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    let stats = pool.stats();
    assert_eq!(stats.acquires, 40);
}

#[test]
fn test_pool_stats_hit_rate() {
    let pool: ObjectPool<u32> = ObjectPool::new(|| 0);

    pool.release(1);
    let _ = pool.acquire();
    let _ = pool.acquire();

    let stats = pool.stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 1);
    assert!((stats.hit_rate() - 0.5).abs() < 0.01);
}
