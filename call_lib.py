from rs_example import tanimoto_search
import numpy as np


fps = np.asarray(
    [
        [4,18014398510563328,70368748371968,0,5242880,32,2147483680,0,0,2048,0,9007199254740992,512,16777216,0,16777216,0,1610612736,137438953472,32768,4398046511104,0,138412032,2147483648,1,0,0,1073741856,72567767498752,5368709128,70368744177668,9223372036854775808,0,35,],
        [10,2305843009230471168,1048576,0,4,16384,288231475663339520,0,281474976710656,0,1024,0,2147483649,105553116266496,1048576,42949672960,1099512676352,576742227280134144,9223372174360838144,36028797019029504,9223372036854775808,2199023255552,1126174918967296,4503608217305088,0,67108864,140737488355328,0,2199023255552,0,9077567998918656,18,0,39,],
    ],dtype="<u8",
)

sims = tanimoto_search(fps, fps[0])
for r in sims:
    print(r.mol_id, r.sim)
