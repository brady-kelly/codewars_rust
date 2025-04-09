pub(crate) fn solve(arr: &[u32]) -> Vec<u32> {
    let mut doms = Vec::<u32>::new();
    for x in 0..arr.len() {
        let mut dominant = true;
        for r in x+1..arr.len() {
            if arr[r] > arr[x] 
            {
                dominant = false;
                break;
            }
        }
        if dominant {
            if (!doms.contains(&arr[x])) {
                doms.push(arr[x]);
            }
        }
    }
    return doms;
}


fn main() {
    solve(&[1,21,4,7,5]);
}