import { useEffect, useState } from "react";
import SelectTable from "../managing/SelectTable";
import { SelectTableRowProps } from "../managing/ManageLicenseAccess";
import CreateLicenseForm from "./CreateLicenseForm";

type LicenseProps = {
  licenseId: number;
  companyId: number;
  companyName: string;
  productId: string;
  productName: string;
};

export default function AdminCompanyLicenses() {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const [licenses, setLicenses] = useState<SelectTableRowProps[]>([]);

  const [invalidatedLicenses] = useState<Set<number>>(new Set());

  const fetchLicenses = async () => {
    const response = await fetch(`${baseUrl}/api/licenses_vital`);
    const data = await response.json();
    const licenses: LicenseProps[] = data.map((license: any) => {
      return {
        licenseId: license.license_id,
        companyId: license.company_id,
        companyName: license.company_name,
        productId: license.product_id,
        productName: license.display_name,
      };
    });
    return licenses;
  };

  const updateChangedOnInvalidate = (license: any) => {
    invalidatedLicenses.add(license.id);
  };

  const invalidateLicense = (index: number) => {
    let license = licensesList.rows[index];

    setLicenses([
      ...licensesList.rows.slice(0, index),
      ...licensesList.rows.slice(index + 1),
    ]);

    updateChangedOnInvalidate(license);
  };

  const invalidateSelectedLicenses = (indices: number[]) => {
    let sortedIndices = indices.sort((a, b) => a - b);

    for (let i = sortedIndices.length - 1; i >= 0; i--) {
      let index = sortedIndices[i];
      let user = licensesList.rows[index];
      licensesList.rows = [
        ...licensesList.rows.slice(0, index),
        ...licensesList.rows.slice(index + 1),
      ];

      updateChangedOnInvalidate(user);
    }

    setLicenses(licensesList.rows);
  };

  const licensesList = {
    header: {
      columns: [{ text: "ID" }, { text: "Company" }, { text: "Product" }],
    },
    rows: licenses,
    button: { text: "Invalidate", action: invalidateLicense },
    outsideButtons: [
      { text: "Invalidate licenses", action: invalidateSelectedLicenses },
    ],
  };

  const patchInvalidated = () => {
    if (invalidatedLicenses.size > 0) {
      fetch(`${baseUrl}/api/licenses`, {
        method: "PATCH",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          licenses: Array.from(invalidatedLicenses, (item: number) => {
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
        .catch(() => console.error("Failed to save license validation status"));
    }
  };

  const handleSave = () => {
    patchInvalidated();
  };

  useEffect(() => {
    fetchLicenses()
      .then((licenses) => {
        setLicenses(
          licenses.map((license) => {
            return {
              id: license.licenseId.toString(),
              columns: [
                { text: license.licenseId.toString() },
                { text: license.companyName },
                { text: license.productName },
              ],
            };
          })
        );
      })
      .catch(() => console.error("Failed to load licenses"));
  }, []);

  return (
    <>
      <section className="container left-aligned">
        <h1>Manage company licenses</h1>
        <SelectTable
          header={licensesList.header}
          rows={licensesList.rows}
          button={licensesList.button}
          outsideButtons={licensesList.outsideButtons}
        />
      </section>
      <section className="container left-aligned button-container">
        <button onClick={handleSave} className="default-button small-button">
          Save changes
        </button>
      </section>
      <section className="center-container">
        <CreateLicenseForm></CreateLicenseForm>
      </section>
    </>
  );
}
