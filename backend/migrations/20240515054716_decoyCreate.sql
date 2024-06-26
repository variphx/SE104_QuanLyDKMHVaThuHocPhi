-- Add migration script here
insert into
    tinh (id, ten)
values
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

insert into
    thanh_pho (id, id_tinh, ten)
values
    ('568', '056', 'Nha Trang');

insert into
    doi_tuong (id, ten, he_so_hoc_phi)
values
    ('_default', 'Đối tượng phổ thông', 1.0),
    ('DTCS001', 'Con/cháu người có công với cách mạng', 0.5),
    ('DTVM001', 'Khu vực 2', 1.0);

insert into
    doi_tuong_chinh_sach (id)
values
    ('_default'),
    ('DTCS001');

insert into
    doi_tuong_vung_mien (id, id_thanh_pho)
values
    ('DTVM001', '568');

insert into
    khoa (id, ten)
values
    ('CS', 'Khoa học Máy tính'),
    ('CE', 'Kỹ thuật Máy tính'),
    ('SE', 'Công nghệ Phần mềm');

insert into
    nganh (id, id_khoa, ten)
values
    ('CSCQ', 'CS', 'Khoa học Máy tính (Chính quy)'),
    ('CSAI', 'CS', 'Trí tuệ Nhân tạo'),
    ('CSTN', 'CS', 'Khoa học Máy tính (Chương trình tài năng)'),
    ('CECQ', 'CE', 'Kỹ thuật Máy tính (Chính quy)'),
    ('SECQ', 'SE', 'Công nghệ Phần mềm (Chính quy)');

insert into
    hoc_ky (id, ten, nam_hoc)
values
    ('202201', 'Một', 2022),
    ('202202', 'Hai', 2022),
    ('202203', 'Hè', 2022),
    ('202301', 'Một', 2023),
    ('202302', 'Hai', 2023),
    ('202303', 'Hè', 2023);

insert into
    chuong_trinh_hoc (id, id_nganh, id_hoc_ky)
values
    ('CSCQ202201', 'CSCQ', '202201');

insert into
    sinh_vien (id, ten, ngay_sinh, id_gioi_tinh, id_que_quan, id_doi_tuong, id_chuong_trinh_hoc)
values
    ('2022010001', 'Huỳnh Anh Dũng', '2004-09-06', 'Nam', '568', '_default', 'CSCQ202201');

insert into
    mon_hoc (id, id_khoa, ten, loai, so_tiet, so_tin_chi)
values
    ('CS112', 'CS', 'Phân tích Thiết kế Thuật toán', 'LT', 60, 4);

insert into
    dang_ky_mon_hoc (id, id_sinh_vien, id_hoc_ky)
values
    ('20220001202201', '20220001', '202201');

insert into
    tham_so (
        id,
        gia_tin_chi_lt,
        gia_tin_chi_th,
        he_so_tin_chi_lt,
        he_so_tin_chi_th,
        sinh_vien_len,
        id_hoc_ky,
        is_mo_dkmh
    )
values
    (1, 27000, 37000, 15, 30, 0, '202201', true);

insert into
    users (username, password)
values
    ('admin', '$argon2id$v=19$m=19456,t=2,p=1$YH9SQNsD49f1V1YL6cSLOA$RJTC++oCDiYEjDPxQKyN0XnrG0lmzVjlL/jsXYNLBr8'),
    ('20220001', '$argon2id$v=19$m=1945,t=2,p=1$ZXJ3Mnl6ekduNU03QkxGRA$ZXaH/ImJpcGN2hoJV1GVwXC1P3KMYzJWMBj2OgTnPmo');
