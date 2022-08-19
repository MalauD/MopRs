import React from 'react';
import PropTypes from 'prop-types';
import { Col, Image as ImgBootstrap, Row } from 'react-bootstrap';
import { SortableHandle } from 'react-sortable-hoc';
import MoreButtonMusic from './Helper/MoreButtonMusic';
import Icon from '../Helper/Icon';

const DragHandle = SortableHandle(() => (
    <Icon
        dataEva="menu-outline"
        evaOptions={{ fill: '#d6d6d6ff', width: '30px', height: '30px' }}
    />
));

function MusicItemRow({
    onClick,
    ImageDz,
    Title,
    Artist,
    children,
    AccessoryRight,
    UseDragHandle,
}) {
    return (
        <tr className="w-100 m-0 p-0 PointerCursor MusicItemRow">
            <td
                className="p-0 py-2 pl-0 align-middle"
                style={{
                    width: '10px',
                }}
            >
                {UseDragHandle && <DragHandle />}
            </td>
            <td className="p-0 py-2 pl-0 align-middle" onClick={onClick} style={{ width: '50px' }}>
                <ImgBootstrap className="PlayerImage my-auto" rounded height="50em" src={ImageDz} />
            </td>
            <td className="p-0 py-2 pl-3 pr-0 align-top" onClick={onClick}>
                <Col className="pl-0">
                    <Row className="p-0 m-0 pt-1">
                        <h6 className="p-0 m-0 lead">{Title}</h6>
                    </Row>
                    <Row className="p-0 m-0">
                        <p className="text-middle p-0 m-0 text-muted">{Artist}</p>
                    </Row>
                </Col>
            </td>
            {AccessoryRight}
            {children?.type || children?.length > 0 ? (
                <td className="align-middle py-3 pr-1 Accessory">
                    <MoreButtonMusic>{children}</MoreButtonMusic>
                </td>
            ) : null}
        </tr>
    );
}

MusicItemRow.propTypes = {
    onClick: PropTypes.func.isRequired,
    ImageDz: PropTypes.string,
    Title: PropTypes.string.isRequired,
    Artist: PropTypes.string.isRequired,
    children: PropTypes.oneOfType([PropTypes.arrayOf(PropTypes.node), PropTypes.node]),
    AccessoryRight: PropTypes.node,
    UseDragHandle: PropTypes.bool,
};

MusicItemRow.defaultProps = {
    ImageDz: '/Ressources/noMusic.jpg',
    children: null,
    AccessoryRight: null,
    UseDragHandle: false,
};

export default MusicItemRow;
