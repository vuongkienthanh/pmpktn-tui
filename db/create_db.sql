CREATE TABLE IF NOT EXISTS patients (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  gender INTEGER NOT NULL,
  birthday TEXT NOT NULL,
  address TEXT,
  past_history TEXT --bệnh nền
);

CREATE INDEX IF NOT EXISTS patient_name
  ON patients (name);

CREATE TABLE IF NOT EXISTS visits (
  id INTEGER PRIMARY KEY,
  exam_date TEXT NOT NULL,
  note TEXT, --bệnh sử
  diagnosis TEXT NOT NULL,
  weight NUMERIC,
  days INTEGER NOT NULL,
  patient_id INTEGER,
  FOREIGN KEY (patient_id)
    REFERENCES patients (id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);

--Kho thuốc
CREATE TABLE IF NOT EXISTS warehouse (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  element TEXT NOT NULL, --thành phần thuốc
  quantity INTEGER NOT NULL, --số lượng trong kho
  usage_unit TEXT NOT NULL, --đơn vị sử dụng
  sale_unit TEXT NOT NULL, --đơn vị bán
  sale_price TEXT NOT NULL, --giá bán
  usage TEXT NOT NULL --cách sử dụng
);

--để search trong app
CREATE INDEX IF NOT EXISTS drug_name
  ON warehouse(name);
CREATE INDEX IF NOT EXISTS drug_element 
  ON warehouse(element);

--Toa thuốc
CREATE TABLE IF NOT EXISTS linedrugs (
  id INTEGER PRIMARY KEY,
  drug_id INTEGER,
  dosage_per NUMERIC NOT NULL, --liều 1 cữ
  times INTEGER NOT NULL,--số cữ
  quantity INTEGER NOT NULL, --số lượng bán ra
  usage TEXT NOT NULL, --cách dùng
  visit_id INTEGER,
  FOREIGN KEY (visit_id)
    REFERENCES visits (id)
      ON DELETE CASCADE
      ON UPDATE CASCADE
);
;
