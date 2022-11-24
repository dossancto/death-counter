import { DisplayInfo } from "../../Model/DisplayInfo"

export function SelectDisplay(props: {display: DisplayInfo[]}) {
  return (
    <select className="selectDisplay">
      {props.display.map((display, i) =>
        <option key={i}>{display.width}x{display.height} -- {display.id}</option>)
      }
    </select>
  )
}
