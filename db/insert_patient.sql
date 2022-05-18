INSERT INTO patients (
  name
  gender
  birthday
  address
  past_history
) VALUES (
?,?,?,?,?
);


INSERT INTO visits (
  exam_date,
  note,
  diagnosis,
  weight,
  days,
  patient_id
) VALUES (
?,?,?,?,?,?
);



INSERT INTO linedrugs (
  drug_id,
  dosage_per,
  times,
  quantity,
  usage,
  visit_id
) VALUES (
?,?,?,?,?,?
);
