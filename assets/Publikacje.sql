CREATE TABLE autorstwa (
    praca NUMBER(3, 0),
    autor VARCHAR2(20) NOT NULL
);

CREATE TABLE autorzy (
    autor VARCHAR2(20) PRIMARY KEY
    ryzyko NUMBER(1, 0) NOT NULL,
    sloty VARCHAR2(4) NOT NULL
);

CREATE TABLE prace (
    id NUMBER(3 , 0) PRIMARY KEY
    tytul VARCHAR2() NOT NULL,
    rok NUMBER(4, 0) NOT NULL,
    autorzy: NUMBER(2, 0) NOT NULL,
    punkty: NUMBER(3, 0) NOT NULL
);
