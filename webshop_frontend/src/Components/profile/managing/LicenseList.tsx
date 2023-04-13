import { useEffect, useState } from "react";
import { License, Product } from "../../../Interfaces";
import LicenseListRow from "./LicenseListRow";
import { fetchCompanyLicenses, fetchProduct } from "../../../ApiController";

type LicenseListProps = {
  companyId: number;
};

/**
 * A list of all owned licenses for a user, and their details.
 *
 * @returns A License List component as a JSX element.
 */
export default function LicenseList(props: LicenseListProps) {
  const [licenses, setLicenses] = useState<License[]>([]);

  useEffect(() => {
    fetchCompanyLicenses(props.companyId).then((licenses) => {
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
