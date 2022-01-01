use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use numpy::*;
extern crate ndarray;

#[pyclass]
pub struct SimRes {
    #[pyo3(get)]
    pub mol_id: u64,
    #[pyo3(get)]
    pub sim: f64,
}
#[pymethods]
impl SimRes {
    #[new]
    pub fn new(mol_id: u64, sim: f64) -> SimRes {
        SimRes { mol_id, sim }
    }
    fn __str__(&self) -> PyResult<String>   {
        Ok(format!("mol_id: {}, similarity: {}", self.mol_id, self.sim))
    }
}

#[pyfunction]
fn tanimoto_search(db: &PyArray2<u64>, query: &PyArray1<u64>) -> PyResult<Vec<SimRes>> {
    let rlen = db.shape()[1];
    let popcnt_idx = rlen - 1;
    let qq = query.readonly();
    let q = qq.as_array();
    let mut results :Vec<SimRes> = Vec::new();
    for current in db.readonly().as_slice().unwrap().chunks(rlen) {
        let mut c_popcnt: u64 = 0;
        for i in 1..popcnt_idx {
            c_popcnt += u64::from((&q[i] & &current[i]).count_ones());
        }
        let res = SimRes{mol_id: current[0], sim: c_popcnt as f64 / (current[popcnt_idx] + q[popcnt_idx] - c_popcnt) as f64};
        results.push(res);
    }
    Ok(results)
}

#[pymodule]
fn rs_example(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tanimoto_search, m)?)?;
    m.add_class::<SimRes>()?;
    Ok(())
}
