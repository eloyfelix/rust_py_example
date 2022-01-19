use numpy::{PyArray1, PyArray2};
use pyo3::{
    pyclass, pyfunction, pymethods, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python,
};

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
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("mol_id: {}, similarity: {}", self.mol_id, self.sim))
    }
}

/// tanimoto_search(db, query, /)
/// --
///
/// This function does a Tanimoto similatiry search.
#[pyfunction]
fn tanimoto_search(db: &PyArray2<u64>, query: &PyArray1<u64>) -> PyResult<Vec<SimRes>> {
    let rlen = db.shape()[1];
    let popcnt_idx = rlen - 1;
    let q = query.readonly();
    let q = q.as_array();
    let mut results: Vec<SimRes> = Vec::new();
    for curr in db.readonly().as_slice()?.chunks(rlen) {
        let mut cum: u64 = 0;
        for i in 1..popcnt_idx {
            cum += u64::from((&q[i] & &curr[i]).count_ones());
        }
        let res = SimRes {
            mol_id: curr[0],
            sim: cum as f64 / (curr[popcnt_idx] + q[popcnt_idx] - cum) as f64,
        };
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
