# Python module in Rust example

New Year's resolution. To run it:

```bash
git clone git@github.com:eloyfelix/rust_py_example.git
cd rust_py_example
python -m venv .env
source .env/bin/activate
pip install maturin tables
maturin develop
python call_lib.py
```
