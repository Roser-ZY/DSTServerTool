import { Badge, Box, Button, Card, Flex, Heading, Text } from "@radix-ui/themes";

const features = [
  {
    title: "Mod 自动更新",
    desc: "检测 Steam Workshop 变更并同步到服务器，避免手动排查版本差异。",
  },
  {
    title: "存档管理",
    desc: "可视化切换世界存档、备份与还原，支持一键回滚。",
  },
  {
    title: "自动部署与启动",
    desc: "按系统分支执行部署流程，保持一致性与可复现性。",
  },
];

export default function App() {
  return (
    <Box className="app-shell">
      <Flex className="hero" direction="column" gap="5">
        <Badge size="3" className="hero-badge">DST Dedicated Server</Badge>
        <Heading size="9">自动化部署与运维桌面工具</Heading>
        <Text size="4" className="hero-subtitle">
          Rust + Tauri 驱动核心流程，前端聚焦可视化、可靠与可控。
        </Text>
        <Flex gap="3" wrap="wrap">
          <Button size="4">初始化部署</Button>
          <Button size="4" variant="surface">导入现有存档</Button>
        </Flex>
      </Flex>

      <Flex className="feature-grid" gap="4" wrap="wrap">
        {features.map((feature) => (
          <Card key={feature.title} className="feature-card" size="3">
            <Flex direction="column" gap="3">
              <Heading size="5">{feature.title}</Heading>
              <Text size="3" className="feature-desc">{feature.desc}</Text>
            </Flex>
          </Card>
        ))}
      </Flex>

      <Card className="status-card" size="3">
        <Flex align="center" justify="between" wrap="wrap" gap="3">
          <Box>
            <Heading size="4">系统状态</Heading>
            <Text size="3" className="muted">等待首次部署配置</Text>
          </Box>
          <Button variant="outline">配置部署流程</Button>
        </Flex>
      </Card>
    </Box>
  );
}
