import LicenseListRow from "./LicenseListRow";
import { LicenseProps } from "../MyAccount";

type LicenseListProps = {
  licenses: LicenseProps[];
};

/**
 * A list of all owned licenses for a user, and their details.
 *
 * @returns A License List component as a JSX element.
 */
export default function LicenseList({ licenses }: LicenseListProps) {
  return (
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
        {licenses.map((license, index) => (
          <LicenseListRow key={index} license={license} />
        ))}
      </tbody>
    </table>
  );
}
