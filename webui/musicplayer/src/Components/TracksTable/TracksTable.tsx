import { useTheme } from "@emotion/react";
import styled from "@emotion/styled";
import { Play } from "@styled-icons/ionicons-sharp";
import { TableBuilder, TableBuilderColumn } from "baseui/table-semantic";
import _ from "lodash";
import { FC } from "react";
import { Link } from "react-router-dom";
import ContextMenu from "../ContextMenu";
import Speaker from "../Icons/Speaker";
import TrackIcon from "../Icons/Track";

const TableWrapper = styled.div`
  margin-top: 31px;
`;

const AlbumCoverAlt = styled.div<{ current: boolean }>`
  height: 43px;
  width: 43px;
  background-color: ${(props) => props.theme.colors.cover};
  display: flex;
  justify-content: center;
  align-items: center;
  margin-right: 10px;
  ${({ current }) => `opacity: ${current ? 0 : 1};`}
`;

const CellWrapper = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  height: 45px;
  color: ${({ theme }) => theme.colors.text};
`;

const AlbumCover = styled.img<{ current: boolean }>`
  height: 43px;
  width: 43px;
  margin-right: 10px;
  ${({ current }) => `opacity: ${current ? 0.4 : 1};`}
`;

const convertToLink = (row: any, item: string, color: string) => {
  switch (item) {
    case "Artist":
      return (
        <Link to={`/artists/${_.get(row, "artistId", "")}`} style={{ color }}>
          {_.get(row, _.toLower(item), "")}
        </Link>
      );
    case "Album":
      return (
        <Link to={`/albums/${_.get(row, "albumId", "")}`} style={{ color }}>
          {_.get(row, _.toLower(item), "")}
        </Link>
      );
    default:
      return _.get(row, _.toLower(item), "");
  }
};

export type CellProps = {
  current?: string | boolean;
  row: any;
  item: any;
  index: number;
  onPlayTrack: (id: string, postion?: number) => void;
  isAlbumTracks: boolean;
};

const Cell: FC<CellProps> = ({
  current,
  row,
  item,
  onPlayTrack,
  index,
  isAlbumTracks,
}) => {
  const theme = useTheme();
  return (
    <CellWrapper>
      {!isAlbumTracks && item === "Title" && !row.cover && (
        <AlbumCoverAlt className="album-cover" current={!!current}>
          <TrackIcon width={24} height={24} color="#a4a3a3" />
        </AlbumCoverAlt>
      )}
      {!isAlbumTracks && item === "Title" && row.cover && (
        <AlbumCover
          className="album-cover"
          src={row.cover}
          current={!!current}
        />
      )}
      {current && isAlbumTracks && (
        <div>
          {item === "#" && (
            <div
              style={{
                position: "absolute",
                left: isAlbumTracks ? 20 : 37,
                marginTop: -11,
              }}
            >
              <Speaker color="#ab28fc" />
            </div>
          )}

          {item !== "#" && (
            <div style={{ flex: 1, color: theme.colors.text }}>
              {convertToLink(row, item, theme.colors.text)}
            </div>
          )}
        </div>
      )}
      {current && !isAlbumTracks && (
        <div>
          {item === "Title" && (
            <div
              style={{
                position: "absolute",
                left: isAlbumTracks ? 20 : 37,
              }}
            >
              <Speaker color={row.cover ? "#fff" : "#ab28fc"} />
            </div>
          )}
          <div style={{ flex: 1 }}>
            {convertToLink(row, item, theme.colors.text)}
          </div>
        </div>
      )}
      {!current && !isAlbumTracks && item === "Title" && (
        <div
          onClick={() => onPlayTrack(row.artistId, row.index)}
          className="floating-play"
        >
          <Play size={16} color={row.cover ? "#fff" : "#000"} />
        </div>
      )}
      {!current && item === "#" && (
        <>
          <div
            onClick={() => onPlayTrack(row.albumId, row.index)}
            className="play"
          >
            <Play size={16} />
          </div>
          <div className="tracknumber">
            {convertToLink(row, item, theme.colors.text)}
          </div>
        </>
      )}
      {!current && item !== "#" && (
        <div style={{ flex: 1 }}>
          {convertToLink(row, item, theme.colors.text)}
        </div>
      )}
    </CellWrapper>
  );
};

export type TracksTableProps = {
  tracks: any[];
  header?: string[];
  title?: JSX.Element;
  currentIndex?: number;
  currentTrackId?: string;
  isPlaying?: boolean;
  maxHeight?: string;
  onPlayTrack: (id: string, position?: number) => void;
  onPlayNext: (id: string) => void;
  onCreatePlaylist: (name: string, description?: string) => void;
  onAddTrackToPlaylist: (playlistId: string, trackId: string) => void;
  recentPlaylists: any[];
};

const TracksTable: FC<TracksTableProps> = ({
  tracks,
  header,
  title,
  currentIndex,
  currentTrackId,
  isPlaying,
  maxHeight,
  onPlayTrack,
  onPlayNext,
  onCreatePlaylist,
  onAddTrackToPlaylist,
  recentPlaylists,
}) => {
  const theme = useTheme();
  return (
    <TableWrapper>
      {title}
      <TableBuilder
        data={tracks}
        overrides={{
          Root: {
            style: {
              maxHeight,
              paddingLeft: "10px",
              backgroundColor: theme.colors.background,
            },
          },
          TableHeadCell: {
            style: ({ $col }) => {
              return {
                width:
                  $col.header === "#"
                    ? "10px"
                    : $col.header === "Time"
                    ? "98px"
                    : $col.header === ""
                    ? "50px"
                    : $col.header === "Title"
                    ? "calc(100% - 200px)"
                    : $col.header === "Artist"
                    ? "100px"
                    : "auto",
                outlineWidth: "0px",
                borderBottomColor: `${theme.colors.background} !important`,
                color: theme.colors.secondaryText,
                backgroundColor: theme.colors.background,
              };
            },
          },
          TableBodyCell: {
            style: ({ $col }) => ({
              outlineWidth: "0px 0px 0px 0px",
              borderBottomColor: `${theme.colors.background} !important`,
              overflow: "hidden",
              whiteSpace: "nowrap",
              textOverflow: "ellipsis",
              maxWidth:
                $col.header === "Artist" || $col.header === "Album"
                  ? "120px"
                  : "300px",
            }),
          },
          TableHead: {
            style: () => ({
              outlineWidth: "0px",
              borderBottomColor: theme.colors.background,
              backgroundColor: theme.colors.background,
            }),
          },
          TableBody: {
            style: () => ({
              border: "none",
              backgroundColor: theme.colors.background,
            }),
          },
        }}
      >
        {header?.map((item, index) => (
          <TableBuilderColumn key={index} header={item}>
            {(row: any) => {
              const current =
                (item === "Title" || item === "#") &&
                ((currentIndex && currentIndex === row.index) ||
                  (currentTrackId && row.id === currentTrackId)) &&
                isPlaying;
              return (
                <Cell
                  current={current}
                  row={row}
                  item={item}
                  index={index}
                  onPlayTrack={onPlayTrack}
                  isAlbumTracks={header.includes("#")}
                />
              );
            }}
          </TableBuilderColumn>
        ))}
        <TableBuilderColumn header="">
          {(row: any) => (
            <CellWrapper>
              <ContextMenu
                track={row}
                onPlayNext={onPlayNext}
                onCreatePlaylist={onCreatePlaylist}
                recentPlaylists={recentPlaylists}
                onAddTrackToPlaylist={onAddTrackToPlaylist}
              />
            </CellWrapper>
          )}
        </TableBuilderColumn>
      </TableBuilder>
    </TableWrapper>
  );
};

TracksTable.defaultProps = {
  header: ["Title", "Artist", "Album", "Time"],
  title: <div />,
  maxHeight: "calc(100vh - 250px)",
};

export default TracksTable;
