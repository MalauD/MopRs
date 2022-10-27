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
        Musics: PropTypes.arrayOf(
            PropTypes.shape({
                _id: PropTypes.number.isRequired,
            })
        ).isRequired,
        Actions: PropTypes.func,
    };

    static defaultProps = {
        Actions: DefaultActions,
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
        const { Musics } = this.props;
        const { RelatedMusicsData } = this.state;
        const prevIds = prevProps.Musics.map((music) => music._id);
        const currentIds = Musics.map((music) => music._id);
        if (!isEqual([...prevIds].sort(), [...currentIds].sort())) {
            const diff = difference(currentIds, prevIds);
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
        const { Musics } = this.props;
        const MusicIds = Musics.map((m) => m._id);
        this.setState({
            isLoading: true,
        });
        axios.post('/Music/Related', { MusicIds, Exclude: MusicIds }).then((res) => {
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
        const { Actions, ...props } = this.props;

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
                title="Related"
                isLoading={isLoading}
                Accessories={Accessories}
                Actions={Actions}
            />
        );
    }
}
