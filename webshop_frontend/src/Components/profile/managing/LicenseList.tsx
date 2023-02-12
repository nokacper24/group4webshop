import LicenseListRow from "./LicenseListRow";

/**
 * List of licenses (placeholder).
 */
const licenses = [
  {
    id: "1",
    name: "E-mail",
    activeUsers: 5,
    total: 5,
    status: "Expired",
    details: ["01.07.2022", "01.01.2023"],
  },
  {
    id: "2",
    name: "Planner",
    activeUsers: 14,
    total: 15,
    status: "Active",
    details: ["01.01.2023", "01.07.2023"],
  },
  {
    id: "3",
    name: "Legal",
    activeUsers: 11,
    total: 50,
    status: "Active",
    details: ["01.01.2023", "01.07.2023"],
  },
  {
    id: "4",
    name: "3D Model",
    activeUsers: 28,
    total: 40,
    status: "Active",
    details: ["01.01.2023", "01.07.2023"],
  },
  {
    id: "5",
    name: "Tax",
    activeUsers: 1,
    total: 5,
    status: "Active",
    details: ["01.01.2023", "01.07.2023"],
  },
];

/**
 * A list of all owned licenses for a user, and their details.
 *
 * @returns A License List component as a JSX element.
 */
export default function LicenseList() {
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
        {licenses.map((license) => (
          <LicenseListRow key={license.name} license={license} />
        ))}
      </tbody>
    </table>
  );
}
