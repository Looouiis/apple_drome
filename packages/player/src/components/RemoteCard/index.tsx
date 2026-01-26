import { Card, ContextMenu, Flex, Text } from "@radix-ui/themes";
import { type PropsWithChildren, forwardRef, useMemo } from "react";
import { Trans, useTranslation } from "react-i18next";
import { Link } from "react-router-dom";
import { type Remote, db } from "../../dexie.ts";
import { PlaylistCover } from "../PlaylistCover/index.tsx";

export const RemoteCard = forwardRef<
	HTMLDivElement,
	PropsWithChildren<{
		remote: Remote;
	}>
>(({ remote, children }, ref) => {
	const { t } = useTranslation();
	// const songAmount = remote.songIds.length;
	// const createTime = useMemo(() => {
	// 	const today = new Date();
	// 	const createTime = new Date(remote.createTime);
	// 	if (today.toDateString() === createTime.toDateString())
	// 		return createTime.toLocaleTimeString();

	// 	return createTime.toLocaleDateString();
	// }, [remote.createTime]);
	return (
		<ContextMenu.Root>
			<ContextMenu.Trigger>
				<Card asChild size="2" mb="4" key={remote.id} ref={ref}>
					<Link to={`/playlist/${remote.id}`}>
						<Flex align="center" gap="4">
							{/*<PlaylistCover playlistId={remote.id} />*/}
							<Flex direction="row" gap="1" flexGrow="1">
								<Text>{remote.name}</Text>
								{/*<Text color="gray" size="2">
									<Flex gap="2">
										{t(
											"page.main.playlistCard.songCount",
											"{songAmount} 首歌曲",
											{
												songAmount,
											},
										)}
										<div>-</div>
										{t(
											"page.main.playlistCard.createTime",
											"创建于 {createTime}",
											{
												createTime,
											},
										)}
									</Flex>
								</Text>*/}
							</Flex>
							{children}
						</Flex>
					</Link>
				</Card>
			</ContextMenu.Trigger>
			<ContextMenu.Content>
				<ContextMenu.Item onSelect={() => {}}>
					<Trans i18nKey="page.main.playlistMenu.play">播放此列表</Trans>
				</ContextMenu.Item>
				<ContextMenu.Item onSelect={() => {}}>
					<Trans i18nKey="page.main.playlistMenu.playShuffled">
						以乱序播放此列表
					</Trans>
				</ContextMenu.Item>
				<ContextMenu.Separator />
				<ContextMenu.Item
					color="red"
					onSelect={() => db.remote.delete(remote.id)}
				>
					<Trans i18nKey="page.main.playlistMenu.delete">删除</Trans>
				</ContextMenu.Item>
			</ContextMenu.Content>
		</ContextMenu.Root>
	);
});
