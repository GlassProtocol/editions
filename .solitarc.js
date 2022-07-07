const path = require('path');
const programDir = path.join(__dirname, 'editions', 'program');
const idlDir = path.join(__dirname, 'target', 'idl');
const sdkDir = path.join(__dirname, 'editions', 'typescript', 'src', 'generated');
const binaryInstallDir = path.join(__dirname, '.crates');

module.exports = {
  idlGenerator: 'anchor',
  programName: 'editions',
  programId: 'hausS13jsjafwWwGqZTUQRmWyvyxn9EQpqMwV1PBBmk',
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};