const clarityTypes = require("./native")
const process = require("process")

const contractPath = "./testdata/contracts/contract.clar"

try {
    const res = clarityTypes.parse(contractPath)
    process.stdout.write(res)
} catch (error) {
    console.log("err type:", typeof error)
    process.stderr.write(`${error}`)
    process.exit(1)
}
