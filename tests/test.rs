use vtflib2::{ImageFormat, VtfFile};

#[test]
fn vtf_create_empty() {
    let mut vtf = VtfFile::new();

    vtf.new_empty(512, 512).null_data(true).create().unwrap();
    assert!(vtf.has_image());
}

#[test]
fn vtf_load_save() {
    let mut vtf = VtfFile::new();

    let original = include_bytes!("test.vtf");

    vtf.load(original).unwrap();
    assert!(vtf.has_image());
    assert_eq!(vtf.width(), 1024);
    assert_eq!(vtf.height(), 1024);
    assert_eq!(vtf.format(), Some(ImageFormat::Dxt1));

    let mut buf = vec![0; vtf.size()];
    vtf.save(buf.as_mut()).unwrap();
    assert_eq!(buf, original);
}

#[test]
fn vtf_two_vtfs() {
    let mut vtf1 = VtfFile::new();
    let mut vtf2 = VtfFile::new();

    vtf1.new_empty(512, 512).null_data(true).create().unwrap();
    assert!(vtf1.has_image());

    vtf2.load(include_bytes!("test.vtf")).unwrap();
    assert_eq!(vtf2.width(), 1024);
    assert_eq!(vtf2.height(), 1024);
    assert_eq!(vtf2.format(), Some(ImageFormat::Dxt1));
}
