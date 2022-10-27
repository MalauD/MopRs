import React from 'react';
import PropTypes from 'prop-types';
import axios from 'axios';
import isEqual from 'lodash.isequal';
import difference from 'lodash.difference';
import ButtonIcon from '../Helper/ButtonIcon';
import MusicGroup from './Groups/MusicGroup';
import { DefaultActions } from '../Items/Actions';

export default class RelatedMusics extends React.Component {
    static propTypes = {
        MusicIds: PropTypes.arrayOf(PropTypes.number).isRequired,
        Actions: PropTypes.func,
        Title: PropTypes.string,
        Limit: PropTypes.number,
        ExcludePassedIds: PropTypes.bool,
    };

    static defaultProps = {
        Actions: DefaultActions,
        Title: 'Related',
        Limit: 20,
        ExcludePassedIds: true
    };

    constructor(props) {
        super(props);
        this.state = {
            RelatedMusicsData: [],
            isLoading: false,
        };
    }

    componentDidMount() {
        this.getNewRelatedMusics();
    }

    componentDidUpdate(prevProps) {
        const { MusicIds } = this.props;
        const { RelatedMusicsData } = this.state;
        const prevIds = prevProps.MusicIds;
        if (!isEqual([...prevIds].sort(), [...MusicIds].sort())) {
            const diff = difference(MusicIds, prevIds);
            if (diff.length > 0) {
                if (RelatedMusicsData.map((m) => m._id).indexOf(diff[0]) === -1) {
                    this.getNewRelatedMusics();
                } else {
                    this.setState((prevState) => ({
                        RelatedMusicsData: prevState.RelatedMusicsData.filter(
                            (m) => m._id !== diff[0]
                        ),
                    }));
                }
            }
        }
    }

    getNewRelatedMusics = () => {
        const { MusicIds, Limit, ExcludePassedIds } = this.props;
        this.setState({
            isLoading: true,
        });
        axios.post('/Music/Related', { MusicIds, Exclude: ExcludePassedIds ? MusicIds : [], Limit }).then((res) => {
            this.setState({
                RelatedMusicsData: res.data.RelatedMusics,
                isLoading: false,
            });
        });
    };

    onReloadRelated = () => {
        this.getNewRelatedMusics();
    };

    render() {
        const { RelatedMusicsData, isLoading } = this.state;
        const { Actions, Title, ...props } = this.props;

        const Accessories = [
            <ButtonIcon
                dataEva="flip-outline"
                onClick={this.onReloadRelated}
                evaOptions={{
                    fill: '#d6d6d6ff',
                    width: '30px',
                    height: '30px',
                }}
            />,
        ];

        return (
            <MusicGroup
                {...props}
                Musics={RelatedMusicsData}
                title={Title}
                isLoading={isLoading}
                Accessories={Accessories}
                Actions={Actions}
            />
        );
    }
}
