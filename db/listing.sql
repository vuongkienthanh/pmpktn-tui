SELECT (id, name, gender, birthday) FROM patients;


SELECT (address, past_history) FROM patients WHERE patients.id = ?;

SELECT * FROM visits WHERE visits.patient_id = ?;

SELECT * FROM linedrugs WHERE linedrugs.visit_id = ?;
