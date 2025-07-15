def bubble_sort(arr, n) {
    let i = 0;

    while i < n {
        let j = 0;
        let limit = n - i - 1;

        while j < limit {
            let jj = j + 1;

            let a = arr[j];
            let b = arr[jj];

            if a > b {
                arr[j] = b;
                arr[jj] = a;
            }

            j = j + 1;
        }

        i = i + 1;
    }

    return arr;
}
