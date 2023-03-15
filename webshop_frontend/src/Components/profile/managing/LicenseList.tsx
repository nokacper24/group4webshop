import { useEffect, useState } from "react";
import { License, Product } from "../../../Interfaces";
import LicenseListRow from "./LicenseListRow";

type LicenseListProps = {
  companyId: number;
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

  const [licenses, setLicenses] = useState<License[]>([]);

  const fetchLicenses = async (companyId: number) => {
    const response = await fetch(
      `${baseUrl}/api/companies/${companyId}/licenses`
    );
    const data = await response.json();
    const licenses: License[] = data.map((license: License) => license);
    return licenses;
  };

  const fetchProduct = async (productId: string) => {
    const response = await fetch(`${baseUrl}/api/products/${productId}`);
    const data = await response.json();
    const product: Product = data;

    return product;
  };

  useEffect(() => {
    fetchLicenses(props.companyId).then((licenses) => {
      licenses.map((license: License) => {
        fetchProduct(license.product_id).then((product: Product) => {
          setLicenses((list) => {
            let newList = list.slice();

            newList.push({
              ...license,
              product_name: product.display_name,
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
