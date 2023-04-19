import { useEffect, useState } from "react";
import SelectTable, {
  SelectTableProps,
  SelectTableRowProps,
} from "../select-table/SelectTable";
import CreateLicenseForm from "./CreateLicenseForm";
import { LicenseVital } from "../../../Interfaces";
import {
  createSelectTableProps,
  createRowProps,
  moveItemBetweenTables,
  moveItemsBetweenTables,
  updateNewChanges,
} from "../select-table/SelectTableFunctions";
import { useNavigate } from "react-router-dom";
import { fetchLicensesVital } from "../../../ApiController";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
// Check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "";
}

/**
 * An admin page for managing companies' licenses.
 * Admin can invalidate licenses, or create new licenses.
 *
 * @returns A page for administrating company licenses.
 */
export default function AdminCompanyLicenses() {
  const navigate = useNavigate();

  const [validLicenses, setValidLicenses] = useState<SelectTableRowProps[]>([]);
  const [invalidLicenses, setInvalidLicenses] = useState<SelectTableRowProps[]>(
    []
  );

  const [newValidatedLicenses] = useState<Set<string>>(new Set());
  const [newInvalidatedLicenses] = useState<Set<string>>(new Set());

  /**
   * Remove a license from list of valid licenses.
   *
   * @param index The index of the license in the list.
   */
  const invalidateLicense = (index: number) => {
    let license = moveItemBetweenTables(
      index,
      validLicensesTable,
      invalidLicensesTable,
      setValidLicenses,
      setInvalidLicenses
    );

    updateNewChanges(license, newValidatedLicenses, newInvalidatedLicenses);
  };

  /**
   * Remove a license from list of invalid licenses.
   *
   * @param index The index of the license in the list.
   */
  const validateLicense = (index: number) => {
    let license = moveItemBetweenTables(
      index,
      invalidLicensesTable,
      validLicensesTable,
      setInvalidLicenses,
      setValidLicenses
    );

    updateNewChanges(license, newInvalidatedLicenses, newValidatedLicenses);
  };

  /**
   * Remove selected licenses from list of valid licenses,
   * and add to list of invalid licenses.
   *
   * @param indices The indices of the licenses in the list.
   */
  const invalidateSelectedLicenses = (indices: number[]) => {
    moveItemsBetweenTables(
      indices,
      validLicensesTable,
      invalidLicensesTable,
      setValidLicenses,
      setInvalidLicenses,
      newValidatedLicenses,
      newInvalidatedLicenses
    );
  };

  /**
   * Remove selected licenses from list of invalid licenses,
   * and add to list of valid licenses.
   *
   * @param indices The indices of the licenses in the list.
   */
  const validateSelectedLicenses = (indices: number[]) => {
    moveItemsBetweenTables(
      indices,
      invalidLicensesTable,
      validLicensesTable,
      setInvalidLicenses,
      setValidLicenses,
      newInvalidatedLicenses,
      newValidatedLicenses
    );
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
      fetch(`${baseUrl}/api/priv/licenses`, {
        method: "PATCH",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          licenses: Array.from(newInvalidatedLicenses, (item: string) => {
            return {
              license_id: parseInt(item.toString()),
              valid: false,
            };
          }),
        }),
      })
        .then((response) => {
          handlePatchLicenseResponse(response);
        })
        .catch(() => alert("Failed to save license validation status"));
    }
  };

  /**
   * Send a PATCH request to set licenses validation to true.
   */
  const patchValidated = () => {
    if (newValidatedLicenses.size > 0) {
      fetch(`${baseUrl}/api/priv/licenses`, {
        method: "PATCH",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          licenses: Array.from(newInvalidatedLicenses, (item: string) => {
            return {
              license_id: parseInt(item.toString()),
              valid: true,
            };
          }),
        }),
      })
        .then((response) => {
          handlePatchLicenseResponse(response);
        })
        .catch(() => alert("Failed to save license validation status"));
    }
  };

  /**
   * Handle the response from a PATCH request sent for patching (saving) licenses.
   *
   * @param response The response from the fetch request.
   */
  const handlePatchLicenseResponse = (response: Response) => {
    if (response.status == 200) {
      // Refresh
      navigate(0);
    } else {
      alert("Something went wrong when saving licenses");
    }
  };

  const handleSave = () => {
    patchInvalidated();
    patchValidated();
  };

  useEffect(() => {
    fetchLicensesVital()
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
