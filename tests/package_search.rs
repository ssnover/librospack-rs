#[test]
fn test_crawl() {
    let packages = librospack::crawl(vec![env!("CARGO_MANIFEST_DIR").into()]);
    assert_eq!(packages.len(), 1);
}
