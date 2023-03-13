import { useEffect, useState } from "react";
import LicenseListRow from "./LicenseListRow";

export type LicenseProps = {
  licenseId: number;
  valid: boolean;
  startDate: Date;
  endDate: Date;
  amount: number;
  companyId: number;
  productId: string;
  productName: string;
};

type LicenseListProps = {
  companyId: string;
};

type Product = {
  name: string;
};

/**
 * A list of all owned licenses for a user, and their details.
 *
 * @returns A License List component as a JSX element.
 */
export default function LicenseList(props: LicenseListProps) {
  let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
  // Check if we are in production mode
  if (import.meta.env.PROD) {
    baseUrl = "";
  }

  const [licenses, setLicenses] = useState<LicenseProps[]>([]);

  const fetchLicenses = async (companyId: string) => {
    const response = await fetch(
      `${baseUrl}/api/companies/${companyId}/licenses`
    );
    const data = await response.json();
    const licenses: LicenseProps[] = data.map((license: any) => {
      return {
        licenseId: license.license_id,
        valid: license.valid,
        startDate: new Date(license.start_date),
        endDate: new Date(license.end_date),
        amount: license.amount,
        productId: license.product_id,
      };
    });

    return licenses;
  };

  const fetchProduct = async (productId: string) => {
    const response = await fetch(`${baseUrl}/api/products/${productId}`);
    const data = await response.json();
    const product: Product = {
      name: data.display_name,
    };

    return product;
  };

  useEffect(() => {
    fetchLicenses(props.companyId).then((licenses) => {
      licenses.map((license: LicenseProps) => {
        fetchProduct(license.productId).then((product: Product) => {
          setLicenses((list) => {
            let newList = list.slice();

            newList.push({
              ...license,
              productName: product.name,
            });

            return newList;
          });
        });
      });
    });
  }, []);

  return (
    <div
      style={{
        maxWidth: "80vw",
        overflow: "auto",
        margin: "0 auto",
      }}
    >
      <table>
        <thead>
          <tr>
            <th>Product</th>
            <th>Active</th>
            <th>Total</th>
            <th>Status</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {licenses?.map((license, index) => (
            <LicenseListRow key={index} license={license} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
