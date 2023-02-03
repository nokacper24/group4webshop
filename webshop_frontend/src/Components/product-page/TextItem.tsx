import {RowItem} from './DescriptionRow'

export const TextItem = (rowItem: RowItem | undefined) => {
    let item: JSX.Element;
    if(rowItem === undefined || rowItem.content === undefined) {
        item = <div className='empty-item'></div>
    } else {
        item = (
        <div className='text-item-wrapper'>
            <h2>
                {rowItem.title}
            </h2>
            <p>
                {rowItem.content}
            </p>
        </div>
        );
    }
    return item;
}