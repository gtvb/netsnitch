const byteUnits = {
    B: 1,
    kB: 1024,
    mB: 1024 * 1024,
    gB: 1024 * 1024 * 1024,
    tB: 1024 * 1024 * 1024 * 1024,
    pB: 1024 * 1024 * 1024 * 1024 * 1024
};

function separateByteString(byteString) {
    const regex = /^(\d+(?:\.\d+)?)\s*([a-zA-Z]+)/;
    const matches = regex.exec(byteString);

    if (matches === null) {
        throw new Error('Invalid byte string format');
    }

    const value = parseFloat(matches[1]);
    const unit = matches[2];

    if (!byteUnits.hasOwnProperty(unit)) {
        throw new Error('Invalid unit');
    }

    return {
        value,
        unit
    }
}

function convertToBytes(byteString) {
    const { value, unit } = separateByteString(byteString);

    return value * byteUnits[unit];
}

function formatByteValue(byteValue) {
    return {
        value: parseFloat(byteValue / (1024 * 1024)),
        unit: "m"
    }
}

export {
    convertToBytes,
    formatByteValue
}
