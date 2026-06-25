import { extractFileSync } from "xberg";

// Test XLSX
const xlsxResult = extractFileSync(
	"/Users/naamanhirschfeld/workspace/xberg-io/xberg/test_documents/xlsx/stanley_cups.xlsx",
);
const fmt = xlsxResult.metadata?.format;
console.log("fmt.excel:", fmt?.excel);
console.log("fmt.docx:", fmt?.docx);
console.log("fmt.pdf:", fmt?.pdf);
console.log("Calling as function?", typeof fmt?.excel);
