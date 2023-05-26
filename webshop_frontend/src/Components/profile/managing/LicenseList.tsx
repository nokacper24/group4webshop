import { useEffect, useState } from "react";
import { FullLicenseInfo } from "../../../Interfaces";
import LicenseListRow from "./LicenseListRow";
import { fetchCompanyLicenses } from "../../../ApiController";

type Props = {
  companyId: number;
};

/**
 * A list of all owned licenses for a user, and their details.
 *
 * @returns A License List component as a JSX element.
 */
export default function LicenseList({ companyId }: Props) {
  const [licenses, setLicenses] = useState<FullLicenseInfo[]>([]);

  useEffect(() => {
    fetchCompanyLicenses(companyId).then((licenses) => {
      setLicenses(licenses);
    });
  }, []);

  return (
    <div className="table-container">
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
