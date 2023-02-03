import {RowItem} from './DescriptionRow'

export const ImageItem = (rowItem: RowItem | undefined) => {
    let item: JSX.Element;
    if(rowItem === undefined ||rowItem.content === undefined) {
        item = <div className='empty-item'></div>
    } else {
        item = (
        <div className='text-item-wrapper'>
            <img src={rowItem.content} alt="" />
        </div>
        );
    }
    return item;
}