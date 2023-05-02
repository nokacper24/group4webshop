import { useEffect, useState } from "react";
import { FullLicenseInfo } from "../../Interfaces";
import { fetchLicensesForUser } from "../../ApiController";

type Props = {
  userId: number;
};

export default function UserLicenses(props: Props) {
  const [licenses, setLicenses] = useState<FullLicenseInfo[]>();

  useEffect(() => {
    fetchLicensesForUser(props.userId.toString()).then((licenses) => {
      setLicenses(licenses);
    });
  }, []);

  return (
    <table>
      <thead>
        <tr>
          <th>Product</th>
          <th>Start</th>
          <th>End</th>
        </tr>
      </thead>
      <tbody>
        {licenses?.map((license) => (
          <tr>
            <td>{license.display_name}</td>
            <td>{new Date(license.start_date).toLocaleDateString()}</td>
            <td>{new Date(license.end_date).toLocaleDateString()}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
