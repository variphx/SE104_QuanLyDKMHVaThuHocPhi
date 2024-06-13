-- Add migration script here
CREATE TABLE
    USERS (
        username TEXT,
        password TEXT NOT NULL,
        PRIMARY KEY (username)
    );

CREATE TABLE
    TINH (id TEXT, ten TEXT NOT NULL, PRIMARY KEY (id));

CREATE TABLE
    THANH_PHO (
        id TEXT,
        id_tinh TEXT NOT NULL,
        ten TEXT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (id_tinh) REFERENCES TINH (id)
    );

CREATE TABLE
    DOI_TUONG (
        id TEXT,
        ten TEXT NOT NULL,
        he_so_hoc_phi DOUBLE PRECISION NOT NULL,
        PRIMARY KEY (id)
    );

CREATE TABLE
    DOI_TUONG_CHINH_SACH (
        id TEXT,
        PRIMARY KEY (id),
        FOREIGN KEY (id) REFERENCES DOI_TUONG (id)
    );

CREATE TABLE
    DOI_TUONG_VUNG_MIEN (
        id TEXT,
        id_thanh_pho TEXT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (id) REFERENCES DOI_TUONG (id),
        FOREIGN KEY (id_thanh_pho) REFERENCES THANH_PHO (id)
    );

INSERT INTO
    DOI_TUONG (id, ten, he_so_hoc_phi)
VALUES
    ('0', 'Đối tượng phổ thông', 1.0);

CREATE TABLE
    KHOA (id TEXT, ten TEXT NOT NULL, PRIMARY KEY (id));

CREATE TABLE
    NGANH (
        id TEXT,
        id_khoa TEXT NOT NULL,
        ten TEXT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (id_khoa) REFERENCES KHOA (id)
    );

CREATE TABLE
    TEN_HOC_KY (ten TEXT, PRIMARY KEY (ten));

INSERT INTO
    TEN_HOC_KY
VALUES
    ('Một'),
    ('Hai'),
    ('Hè');

CREATE TABLE
    HOC_KY (
        id TEXT,
        ten TEXT NOT NULL,
        nam_hoc INT,
        PRIMARY KEY (id),
        FOREIGN KEY (ten) REFERENCES TEN_HOC_KY (ten)
    );

CREATE TABLE
    CHUONG_TRINH_HOC (
        id TEXT,
        id_nganh TEXT NOT NULL,
        id_hoc_ky TEXT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (id_nganh) REFERENCES NGANH (id),
        FOREIGN KEY (id_hoc_ky) REFERENCES HOC_KY (id)
    );

CREATE TABLE
    LOAI_MON_HOC (ten TEXT, PRIMARY KEY (ten));

INSERT INTO
    LOAI_MON_HOC (ten)
VALUES
    ('LT'),
    ('TH');

CREATE TABLE
    MON_HOC (
        id TEXT,
        id_khoa TEXT NOT NULL,
        ten TEXT NOT NULL,
        loai TEXT NOT NULL,
        so_tiet INT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (loai) REFERENCES LOAI_MON_HOC (ten),
        FOREIGN KEY (id_khoa) REFERENCES KHOA (id)
    );

CREATE TABLE
    MON_HOC_MO (
        id TEXT,
        id_mon_hoc TEXT NOT NULL,
        id_hoc_ky TEXT NOT NULL,
        id_chuong_trinh_hoc TEXT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (id_mon_hoc) REFERENCES MON_HOC (id),
        FOREIGN KEY (id_chuong_trinh_hoc) REFERENCES CHUONG_TRINH_HOC (id),
        FOREIGN KEY (id_hoc_ky) REFERENCES HOC_KY (id)
    );

CREATE TABLE
    GIOI_TINH (id TEXT, PRIMARY KEY (id));

INSERT INTO
    GIOI_TINH (id)
VALUES
    ('Nam'),
    ('Nữ');

CREATE TABLE
    SINH_VIEN (
        id TEXT,
        ten TEXT NOT NULL,
        ngay_sinh DATE NOT NULL,
        id_gioi_tinh TEXT NOT NULL,
        id_que_quan TEXT NOT NULL,
        id_doi_tuong TEXT NOT NULL,
        id_chuong_trinh_hoc TEXT NOT NULL,
        PRIMARY KEY (id),
        FOREIGN KEY (id_gioi_tinh) REFERENCES GIOI_TINH (id),
        FOREIGN KEY (id_que_quan) REFERENCES THANH_PHO (id),
        FOREIGN KEY (id_doi_tuong) REFERENCES DOI_TUONG (id),
        FOREIGN KEY (id_chuong_trinh_hoc) REFERENCES CHUONG_TRINH_HOC (id)
    );

CREATE TABLE
    DANG_KY_MON_HOC (
        id TEXT,
        id_sinh_vien TEXT NOT NULL,
        id_hoc_ky TEXT NOT NULL,
        PRIMARY KEY (id),
        UNIQUE (id_sinh_vien, id_hoc_ky)
    );

CREATE TABLE
    CHI_TIET_DANG_KY_MON_HOC (
        id_dang_ky_mon_hoc TEXT NOT NULL,
        id_mon_hoc_mo TEXT NOT NULL,
        PRIMARY KEY (id_dang_ky_mon_hoc, id_mon_hoc_mo),
        FOREIGN KEY (id_mon_hoc_mo) REFERENCES MON_HOC_MO (id),
        FOREIGN KEY (id_dang_ky_mon_hoc) REFERENCES DANG_KY_MON_HOC (id)
    );

CREATE TABLE
    HOC_PHI (
        tong DOUBLE PRECISION NOT NULL,
        da_thanh_toan BIGINT NOT NULL,
        id_hoc_ky TEXT NOT NULL,
        id_sinh_vien TEXT NOT NULL,
        PRIMARY KEY (id_sinh_vien, id_hoc_ky),
        FOREIGN KEY (id_hoc_ky) REFERENCES HOC_KY (id),
        FOREIGN KEY (id_sinh_vien) REFERENCES SINH_VIEN (id)
    );

CREATE TABLE
    THAM_SO (
        id SMALLINT PRIMARY KEY,
        gia_tin_chi_lt BIGINT NOT NULL,
        gia_tin_chi_th BIGINT NOT NULL,
        sinh_vien_len INT NOT NULL,
        id_hoc_ky_hien_tai TEXT NOT NULL,
        FOREIGN KEY (id_hoc_ky_hien_tai) REFERENCES HOC_KY (id)
    );

