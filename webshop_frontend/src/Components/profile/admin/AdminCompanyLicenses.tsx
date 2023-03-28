import { useEffect, useState } from "react";
import SelectTable, {
  SelectTableProps,
  SelectTableRowProps,
} from "../managing/SelectTable";
import CreateLicenseForm from "./CreateLicenseForm";
import { LicenseVital } from "../../../Interfaces";
import {
  createSelectTableProps,
  createRowProps,
} from "../managing/SelectTableFunctions";

/**
 * An admin page for managing companies' licenses.
 * Admin can invalidate licenses, or create new licenses.
 *
 * @returns A page for administrating company licenses.
 */
export default function AdminCompanyLicenses() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const [validLicenses, setValidLicenses] = useState<SelectTableRowProps[]>([]);
  const [invalidLicenses, setInvalidLicenses] = useState<SelectTableRowProps[]>(
    []
  );

  const [newValidatedLiceneses, setNewValidatedLicenses] = useState<
    Set<number>
  >(new Set());
  const [newInvalidatedLicenses] = useState<Set<number>>(new Set());

  /**
   * Get all licenses.
   *
   * @returns All licenses.
   */
  const fetchLicenses = async () => {
    const response = await fetch(`${baseUrl}/api/licenses_vital`);
    const data = await response.json();
    return data.map((license: LicenseVital) => license);
  };

  /**
   * Add license as pending to be invalidated.
   *
   * @param license The license to invalidate.
   */
  const updateChangedOnInvalidate = (license: SelectTableRowProps) => {
    newInvalidatedLicenses.add(parseInt(license.id));
  };

  /**
   * Add license as pending to be validated.
   *
   * @param license The license to validate.
   */
  const updateChangedOnValidate = (license: SelectTableRowProps) => {
    newValidatedLiceneses.add(parseInt(license.id));
  };

  /**
   * Remove a license from list of valid licenses.
   *
   * @param index The index of the license in the list.
   */
  const invalidateLicense = (index: number) => {
    let license = validLicensesTable.rows[index];

    setValidLicenses([
      ...validLicensesTable.rows.slice(0, index),
      ...validLicensesTable.rows.slice(index + 1),
    ]);

    updateChangedOnInvalidate(license);
  };

  /**
   * Remove a license from list of invalid licenses.
   *
   * @param index The index of the license in the list.
   */
  const validateLicense = (index: number) => {
    let license = invalidLicensesTable.rows[index];

    setInvalidLicenses([
      ...invalidLicensesTable.rows.slice(0, index),
      ...invalidLicensesTable.rows.slice(index + 1),
    ]);

    updateChangedOnValidate(license);
  };

  /**
   *  Remove selected licenses from list of valid licenses.
   *
   * @param indices The indices of the licenses in the list.
   */
  const invalidateSelectedLicenses = (indices: number[]) => {
    let sortedIndices = indices.sort((a, b) => a - b);

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = validLicensesTable.rows[index];
      validLicensesTable.rows = [
        ...validLicensesTable.rows.slice(0, index),
        ...validLicensesTable.rows.slice(index + 1),
      ];

      updateChangedOnInvalidate(user);
    }

    setValidLicenses(validLicensesTable.rows);
  };

  const validateSelectedLicenses = (indices: number[]) => {
    let sortedIndices = indices.sort((a, b) => a - b);

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = invalidLicensesTable.rows[index];
      invalidLicensesTable.rows = [
        ...invalidLicensesTable.rows.slice(0, index),
        ...invalidLicensesTable.rows.slice(index + 1),
      ];

      updateChangedOnValidate(user);
    }

    setInvalidLicenses(invalidLicensesTable.rows);
  };

  const validLicensesTable: SelectTableProps = createSelectTableProps(
    ["ID", "Company", "Product"],
    validLicenses,
    "Invalidate",
    invalidateLicense,
    new Map([["Invalidate licenses", invalidateSelectedLicenses]])
  );

  const invalidLicensesTable: SelectTableProps = createSelectTableProps(
    ["ID", "Company", "Product"],
    invalidLicenses,
    "Validate",
    validateLicense,
    new Map([["Validate licenses", validateSelectedLicenses]])
  );

  /**
   * Send a PATCH request to set licenses validation to false.
   */
  const patchInvalidated = () => {
    if (newInvalidatedLicenses.size > 0) {
      fetch(`${baseUrl}/api/licenses`, {
        method: "PATCH",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          licenses: Array.from(newInvalidatedLicenses, (item: number) => {
            return {
              license_id: parseInt(item.toString()),
              valid: false,
            };
          }),
        }),
      })
        .then((response) => {
          const status = response.status;
          if (status == 200) {
            location.reload();
          } else {
            alert("Something went wrong when saving licenses");
          }
        })
        .catch(() => alert("Failed to save license validation status"));
    }
  };

  const handleSave = () => {
    patchInvalidated();
  };

  useEffect(() => {
    fetchLicenses()
      .then((licenses: LicenseVital[]) => {
        let validLicenses: SelectTableRowProps[] = [];
        let invalidLicenses: SelectTableRowProps[] = [];

        licenses.map((license: LicenseVital) => {
          let newLicense = createRowProps(license.license_id.toString(), [
            license.license_id.toString(),
            license.company_name,
            license.display_name,
          ]);

          if (license.valid) {
            validLicenses.push(newLicense);
          } else {
            invalidLicenses.push(newLicense);
          }
        });

        setValidLicenses(validLicenses);
        setInvalidLicenses(invalidLicenses);
      })
      .catch(() => alert("Failed to load licenses"));
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Valid licenses</h1>
        <SelectTable
          header={validLicensesTable.header}
          rows={validLicensesTable.rows}
          button={validLicensesTable.button}
          outsideButtons={validLicensesTable.outsideButtons}
        />
      </section>
      <section className="container left-aligned">
        <h1>Invalid licenses</h1>
        <SelectTable
          header={invalidLicensesTable.header}
          rows={invalidLicensesTable.rows}
          button={invalidLicensesTable.button}
          outsideButtons={invalidLicensesTable.outsideButtons}
        />
      </section>
      <section className="container left-aligned button-container">
        <button onClick={handleSave} className="default-button small-button">
          Save changes
        </button>
      </section>
      <section className="center-container">
        <CreateLicenseForm />
      </section>
    </>
  );
}
