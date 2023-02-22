import { RowTypes } from "./RowTypes"
import { useDrag } from 'react-dnd'

export function DescriptionHeader() {
    const [{isDragging}, drag] = useDrag(() => ({
        type: RowTypes.DescriptionHeader,
        collect: monitor => ({
            isDragging: !!monitor.isDragging(),
        }),
    }))

    return (
        <div className="button-container">
            <button></button>
            <button></button>
        </div>
    )
}