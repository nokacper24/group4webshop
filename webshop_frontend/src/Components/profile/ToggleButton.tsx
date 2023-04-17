import { useState } from "react";

/**
 * The props of the toggle button component.
 * The id is used to identify the button and should be unique.
 * The handleClick function is called when the button is clicked.
 */
export type ToggleButtonProps = {
  id: string;
  checked: boolean;
  handleClick: (checked: boolean, id: string) => void;
};

/**
 * A toggle button component. The button is a switch that can be toggled on or off.
 * The parameter handleClick is called when the button is clicked.
 *
 * @param props the props of the component
 * @returns the toggle button component
 */
export function ToggleButton(props: ToggleButtonProps) {
  const [checked, setChecked] = useState<boolean>(props.checked);

  return (
    <label className="toggle-switch" htmlFor={props.id}>
      <input
        type="checkbox"
        id={props.id}
        defaultChecked={checked}
        onClick={() => {
          setChecked((checked) => !checked);
          props.handleClick(!checked, props.id);
        }}
      />
      <span className="slider"></span>
    </label>
  );
}
