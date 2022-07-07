const path = require('path');
const programDir = path.join(__dirname, 'editions', 'program');
const idlDir = path.join(__dirname, 'target', 'idl');
const sdkDir = path.join(__dirname, 'editions', 'typescript');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'editions',
  programId: 'GeTddTEvfE8My8HNbnSeS1o2tzyn9Z4S194tCUZPbQ8Y',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};