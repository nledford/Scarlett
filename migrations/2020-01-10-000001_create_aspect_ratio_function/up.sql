-- use a javascript function to determine the aspect ratio of the image
create or replace function calculate_aspect_ratio(width integer, height integer) returns varchar as
$$
    // if either number is zero, immediately return
    if (width === 0 || height === 0) {
        return 'N/A'
    }

    // store all valid aspect ratios in a dictionary
    let ratios = {
        // landscape
        '5:4': 5 / 4,
        '4:3': 4 / 3,
        '3:2': 3 / 2,
        '16:10': 16 / 10,
        '5:3': 5 / 3,
        '16:9': 16 / 9,
        '21:9': 21 / 9,
        '21:10': 21 / 10,

        // portrait
        '4:5': 4 / 5,
        '3:4': 3 / 4,
        '2:3': 2 / 3,
        '10:16': 10 / 16,
        '3:5': 3 / 5,
        '9:16': 9 / 16,
        '9:21': 9 / 21,
        '10:21': 10 / 21,

        // default
        '1:1': 1 / 1,
    }

    const findClosestRatio = (ratio) => {
        let lowestDiff = 9999999999
        let bestStd = '1:1'

        for (let key in ratios) {
            const stdRatio = ratios[key]

            const diff = Math.abs(stdRatio - ratio)
            if (diff < lowestDiff) {
                lowestDiff = diff
                bestStd = key
            }
        }

        return bestStd
    }

    const extractRatio = (width, height) => {
        const divided = width / height
        if (divided == 1.0) {
            return '1:1'
        }
        return findClosestRatio(divided)
    }

    return extractRatio(width, height)
    $$
    language plv8
    immutable strict;