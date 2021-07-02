import styled from "styled-components";
import {MEDIUM_GRAY} from "../constants";
import {useTauriEvent} from "../../services/useTauriEvent";

const StatusBarContainer = styled.div({
  backgroundColor: MEDIUM_GRAY,
  fontSize: 10,
  height: 20,
  display: "flex",
  padding: "0 10px 2px 10px",
  alignItems: "center",
});

export default function StatusBar() {
  const statusBarMessage = useTauriEvent<string>("status_bar_change");
  return <StatusBarContainer>{statusBarMessage}</StatusBarContainer>;
}
