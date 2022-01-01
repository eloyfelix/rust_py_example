from rs_example import tanimoto_search
import tables as tb

with tb.open_file("10mols.h5", mode="r") as fp_file:
    fps = fp_file.root.fps[:]
    num_fields = len(fps[0])
    fps = fps.view("<u8")
    fps = fps.reshape(int(fps.size / num_fields), num_fields)

sims = tanimoto_search(fps, fps[0])
for r in sims:
    print(r.mol_id, r.sim)
