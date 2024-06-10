-- Add migration script here
INSERT INTO
    TINH (id, ten)
VALUES
    ('001', 'Hà Nội'),
    ('002', 'Hà Giang'),
    ('004', 'Cao Bằng'),
    ('006', 'Bắc Kạn'),
    ('008', 'Tuyên Quang'),
    ('010', 'Lào Cai'),
    ('011', 'Điện Biên'),
    ('012', 'Lai Châu'),
    ('014', 'Sơn La'),
    ('015', 'Yên Bái'),
    ('017', 'Hòa Bình'),
    ('019', 'Thái Nguyên'),
    ('020', 'Lạng Sơn'),
    ('022', 'Quảng Ninh'),
    ('024', 'Bắc Giang'),
    ('025', 'Phú Thọ'),
    ('026', 'Vĩnh Phúc'),
    ('027', 'Bắc Ninh'),
    ('030', 'Hải Dương'),
    ('031', 'Hải Phòng'),
    ('033', 'Hưng Yên'),
    ('034', 'Thái Bình'),
    ('035', 'Hà Nam'),
    ('036', 'Nam Định'),
    ('037', 'Ninh Bình'),
    ('038', 'Thanh Hóa'),
    ('040', 'Nghệ An'),
    ('042', 'Hà Tĩnh'),
    ('044', 'Quảng Bình'),
    ('045', 'Quảng Trị'),
    ('046', 'Thừa Thiên Huế'),
    ('048', 'Đà Nẵng'),
    ('049', 'Quảng Nam'),
    ('051', 'Quảng Ngãi'),
    ('052', 'Bình Định'),
    ('054', 'Phú Yên'),
    ('056', 'Khánh Hòa'),
    ('058', 'Ninh Thuận'),
    ('060', 'Bình Thuận'),
    ('062', 'Kon Tum'),
    ('064', 'Gia Lai'),
    ('066', 'Đắk Lắk'),
    ('067', 'Đắk Nông'),
    ('068', 'Lâm Đồng'),
    ('070', 'Bình Phước'),
    ('072', 'Tây Ninh'),
    ('074', 'Bình Dương'),
    ('075', 'Đồng Nai'),
    ('077', 'Bà Rịa - Vũng Tàu'),
    ('079', 'Hồ Chí Minh'),
    ('080', 'Long An'),
    ('082', 'Tiền Giang'),
    ('083', 'Bến Tre'),
    ('084', 'Trà Vinh'),
    ('086', 'Đồng Tháp'),
    ('087', 'An Giang'),
    ('089', 'Kiên Giang'),
    ('091', 'Cần Thơ'),
    ('092', 'Hậu Giang'),
    ('093', 'Sóc Trăng'),
    ('094', 'Bạc Liêu'),
    ('095', 'Cà Mau'),
    ('096', 'Thành phố Hồ Chí Minh');

INSERT INTO
    THANH_PHO (id, id_tinh, ten)
VALUES
    ('568', '056', 'Nha Trang');

INSERT INTO
    KHOA (id, ten)
VALUES
    ('CS', 'Khoa học Máy tính');

INSERT INTO
    NGANH (id, id_khoa, ten)
VALUES
    ('CSCQ', 'CS', 'Khoa học Máy tính (Chính quy)');