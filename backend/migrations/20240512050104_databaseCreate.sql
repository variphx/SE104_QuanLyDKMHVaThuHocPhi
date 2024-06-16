-- add migration script here
create table
    users (
        username text,
        password text not null,
        primary key (username)
    );

create table
    session (
        id bigint,
        username text not null,
        primary key (id),
        foreign key (username) references users (username)
    );

create table
    tinh (id text, ten text not null, primary key (id));

create table
    thanh_pho (
        id text,
        id_tinh text not null,
        ten text not null,
        primary key (id),
        foreign key (id_tinh) references tinh (id)
    );

create table
    doi_tuong (
        id text,
        ten text not null,
        he_so_hoc_phi double precision not null,
        primary key (id)
    );

create table
    doi_tuong_chinh_sach (
        id text,
        primary key (id),
        foreign key (id) references doi_tuong (id)
    );

create table
    doi_tuong_vung_mien (
        id text,
        id_thanh_pho text not null,
        primary key (id),
        foreign key (id) references doi_tuong (id),
        foreign key (id_thanh_pho) references thanh_pho (id)
    );

insert into
    doi_tuong (id, ten, he_so_hoc_phi)
values
    ('0', 'Đối tượng phổ thông', 1.0);

create table
    khoa (id text, ten text not null, primary key (id));

create table
    nganh (
        id text,
        id_khoa text not null,
        ten text not null,
        primary key (id),
        foreign key (id_khoa) references khoa (id)
    );

create table
    ten_hoc_ky (ten text, primary key (ten));

insert into
    ten_hoc_ky
values
    ('Một'),
    ('Hai'),
    ('Hè');

create table
    hoc_ky (
        id text,
        ten text not null,
        nam_hoc int,
        primary key (id),
        foreign key (ten) references ten_hoc_ky (ten)
    );

create table
    chuong_trinh_hoc (
        id text,
        id_nganh text not null,
        id_hoc_ky text not null,
        primary key (id),
        foreign key (id_nganh) references nganh (id),
        foreign key (id_hoc_ky) references hoc_ky (id)
    );

create table
    loai_mon_hoc (ten text, primary key (ten));

insert into
    loai_mon_hoc (ten)
values
    ('LT'),
    ('TH');

create table
    mon_hoc (
        id text,
        id_khoa text not null,
        ten text not null,
        loai text not null,
        so_tiet int not null,
        so_tin_chi smallint not null,
        primary key (id),
        foreign key (loai) references loai_mon_hoc (ten),
        foreign key (id_khoa) references khoa (id)
    );

create table
    mon_hoc_mo (
        id text,
        id_mon_hoc text not null,
        id_hoc_ky text not null,
        id_chuong_trinh_hoc text not null,
        primary key (id),
        foreign key (id_mon_hoc) references mon_hoc (id),
        foreign key (id_chuong_trinh_hoc) references chuong_trinh_hoc (id),
        foreign key (id_hoc_ky) references hoc_ky (id)
    );

create table
    gioi_tinh (id text, primary key (id));

insert into
    gioi_tinh (id)
values
    ('Nam'),
    ('Nữ');

create table
    sinh_vien (
        id text,
        ten text not null,
        ngay_sinh date not null,
        id_gioi_tinh text not null,
        id_que_quan text not null,
        id_doi_tuong text not null,
        id_chuong_trinh_hoc text not null,
        primary key (id),
        foreign key (id_gioi_tinh) references gioi_tinh (id),
        foreign key (id_que_quan) references thanh_pho (id),
        foreign key (id_doi_tuong) references doi_tuong (id),
        foreign key (id_chuong_trinh_hoc) references chuong_trinh_hoc (id)
    );

create table
    dang_ky_mon_hoc (
        id text,
        id_sinh_vien text not null,
        id_hoc_ky text not null,
        primary key (id),
        unique (id_sinh_vien, id_hoc_ky)
    );

create table
    chi_tiet_dang_ky_mon_hoc (
        id_dang_ky_mon_hoc text not null,
        id_mon_hoc text not null,
        primary key (id_dang_ky_mon_hoc, id_mon_hoc),
        foreign key (id_mon_hoc) references mon_hoc (id),
        foreign key (id_dang_ky_mon_hoc) references dang_ky_mon_hoc (id)
    );

create table
    hoc_phi (
        tong double precision not null,
        da_thanh_toan double precision not null,
        id_hoc_ky text not null,
        id_sinh_vien text not null,
        primary key (id_sinh_vien, id_hoc_ky),
        foreign key (id_hoc_ky) references hoc_ky (id),
        foreign key (id_sinh_vien) references sinh_vien (id)
    );

create table
    tham_so (
        id smallint primary key,
        gia_tin_chi_lt bigint not null,
        gia_tin_chi_th bigint not null,
        he_so_tin_chi_lt smallint not null,
        he_so_tin_chi_th smallint not null,
        sinh_vien_len int not null,
        id_hoc_ky text not null,
        foreign key (id_hoc_ky) references hoc_ky (id)
    );
