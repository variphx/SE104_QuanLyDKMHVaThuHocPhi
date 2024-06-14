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

INSERT INTO
    HOC_KY (id, ten, nam_hoc)
VALUES
    ('202201', 'Một', 2022);

INSERT INTO
    CHUONG_TRINH_HOC (id, id_nganh, id_hoc_ky)
VALUES
    ('CSCQ202201', 'CSCQ', '202201');

INSERT INTO
    SINH_VIEN (id, ten, ngay_sinh, id_gioi_tinh, id_que_quan, id_doi_tuong, id_chuong_trinh_hoc)
VALUES
    ('20220001', 'Huỳnh Anh Dũng', '2004-09-06', 'Nam', '568', '0', 'CSCQ202201');

INSERT INTO
    MON_HOC (id, id_khoa, ten, loai, so_tiet)
VALUES
    ('CS112', 'CS', 'Phân tích Thiết kế Thuật toán', 'LT', 60);

INSERT INTO
    MON_HOC_MO (id, id_mon_hoc, id_hoc_ky, id_chuong_trinh_hoc)
VALUES
    ('temp', 'CS112', '202201', 'CSCQ202201');

INSERT INTO
    DANG_KY_MON_HOC (id, id_sinh_vien, id_hoc_ky)
VALUES
    ('20220001202201', '20220001', '202201');

INSERT INTO
    HOC_PHI (tong, da_thanh_toan, id_hoc_ky, id_sinh_vien)
VALUES
    (14000000, 0, '202201', '20220001');

INSERT INTO
    THAM_SO (
        id,
        gia_tin_chi_lt,
        gia_tin_chi_th,
        he_so_tin_chi_lt,
        he_so_tin_chi_th,
        sinh_vien_len,
        id_hoc_ky
    )
VALUES
    (1, 27000, 37000, 15, 30, 0, '202201');
