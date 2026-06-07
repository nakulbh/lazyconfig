use gpui::*;

mod actions;
mod components;
mod models;
mod services;
mod state;
mod theme;
mod views;

use models::agent::Subagent;
use models::mcp::MCPServer;
use models::platform::AgentPlatform;
use models::skill::Skill;
use services::scanner::Scanner;
use state::AppState;
use theme::Theme;

struct MainView {
    skills: Vec<Skill>,
    agents: Vec<Subagent>,
    mcp_servers: Vec<MCPServer>,
    active_tab: usize,
    filter_text: String,
    selected_skill: Option<usize>,
}

impl MainView {
    fn new() -> Self {
        let scanner = Scanner::new();
        let skills = scanner.scan_all_skills();
        let agents = scanner.scan_all_agents();
        let mcp_servers = scanner.scan_all_mcp_servers();
        Self {
            skills,
            agents,
            mcp_servers,
            active_tab: 0,
            filter_text: String::new(),
            selected_skill: None,
        }
    }

    fn filtered_skills(&self) -> Vec<&Skill> {
        self.skills
            .iter()
            .filter(|s| {
                if self.filter_text.is_empty() {
                    true
                } else {
                    s.name.to_lowercase().contains(&self.filter_text.to_lowercase())
                        || s.description.to_lowercase().contains(&self.filter_text.to_lowercase())
                }
            })
            .collect()
    }

    fn filtered_agents(&self) -> Vec<&Subagent> {
        self.agents
            .iter()
            .filter(|a| {
                if self.filter_text.is_empty() {
                    true
                } else {
                    a.name.to_lowercase().contains(&self.filter_text.to_lowercase())
                        || a.description.to_lowercase().contains(&self.filter_text.to_lowercase())
                }
            })
            .collect()
    }

    fn filtered_mcp_servers(&self) -> Vec<&MCPServer> {
        self.mcp_servers
            .iter()
            .filter(|m| {
                if self.filter_text.is_empty() {
                    true
                } else {
                    m.name.to_lowercase().contains(&self.filter_text.to_lowercase())
                }
            })
            .collect()
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = div()
            .flex()
            .flex_col()
            .flex_1()
            .id("content-scroll")
            .overflow_scroll()
            .p_4()
            .child(self.render_content(cx));

        let main_area = if self.selected_skill.is_some() {
            div()
                .flex()
                .flex_row()
                .flex_1()
                .child(content)
                .child(self.render_detail_panel(cx))
        } else {
            div().flex().flex_col().flex_1().child(content)
        };

        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(Theme::bg_primary())
            .text_color(Theme::text_primary())
            // Sidebar
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(260.0))
                    .h_full()
                    .bg(Theme::bg_secondary())
                    .border_r_1()
                    .border_color(Theme::border())
                    .p_4()
                    .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).child("lazyconfig"))
                    .child(div().text_xs().text_color(Theme::text_muted()).child("AI Agent Config Manager"))
                    .child(div().mt_6().text_sm().text_color(Theme::text_secondary()).font_weight(FontWeight::SEMIBOLD).child("Global"))
                    .child(div().mt_4().text_sm().text_color(Theme::text_secondary()).font_weight(FontWeight::SEMIBOLD).child("Projects")),
            )
            // Content
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .h_full()
                    // Tab bar
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .border_b_1()
                            .border_color(Theme::border())
                            .child(self.render_tab("Skills", 0, cx))
                            .child(self.render_tab("Subagents", 1, cx))
                            .child(self.render_tab("MCP", 2, cx)),
                    )
                    // Search bar
                    .child(self.render_search_bar(cx))
                    // Main content + detail
                    .child(main_area),
            )
    }
}

impl MainView {
    fn render_tab(&self, label: &str, index: usize, cx: &mut Context<Self>) -> AnyElement {
        let is_active = self.active_tab == index;
        let mut el = div()
            .id(("tab", index))
            .flex()
            .items_center()
            .px_4()
            .py_3()
            .cursor_pointer()
            .text_sm()
            .text_color(if is_active { Theme::text_primary() } else { Theme::text_muted() });

        if is_active {
            el = el.border_b_2().border_color(Theme::border_active());
        }

        el.on_click(cx.listener(move |this: &mut MainView, _event: &ClickEvent, _window, cx| {
            this.active_tab = index;
            this.selected_skill = None;
            cx.notify();
        }))
        .child(label.to_string())
        .into_any_element()
    }

    fn render_search_bar(&self, cx: &mut Context<Self>) -> AnyElement {
        let tab_name = match self.active_tab {
            0 => "skills",
            1 => "subagents",
            2 => "MCP servers",
            _ => "items",
        };
        let count = match self.active_tab {
            0 => self.filtered_skills().len(),
            1 => self.filtered_agents().len(),
            2 => self.filtered_mcp_servers().len(),
            _ => 0,
        };
        let total = match self.active_tab {
            0 => self.skills.len(),
            1 => self.agents.len(),
            2 => self.mcp_servers.len(),
            _ => 0,
        };
        let placeholder = format!("Type to filter {} ({} found)...", tab_name, count);
        let display_text = if self.filter_text.is_empty() {
            placeholder
        } else {
            self.filter_text.clone()
        };
        let text_color = if self.filter_text.is_empty() {
            Theme::text_muted()
        } else {
            Theme::text_primary()
        };

        div()
            .flex()
            .flex_row()
            .items_center()
            .px_4()
            .py_3()
            .gap_3()
            .border_b_1()
            .border_color(Theme::border())
            .child(
                div()
                    .id("search-input")
                    .flex()
                    .items_center()
                    .flex_1()
                    .h(px(32.0))
                    .px_3()
                    .rounded_md()
                    .border_1()
                    .border_color(Theme::border())
                    .bg(Theme::bg_primary())
                    .text_sm()
                    .text_color(text_color)
                    .cursor_text()
                    .on_key_down(cx.listener(|this: &mut MainView, event: &KeyDownEvent, _window, cx| {
                        let key = &event.keystroke.key;
                        if key == "backspace" {
                            this.filter_text.pop();
                            this.selected_skill = None;
                            cx.notify();
                        } else if key == "escape" {
                            this.filter_text.clear();
                            this.selected_skill = None;
                            cx.notify();
                        } else if key.len() == 1
                            && !key.starts_with("ctrl")
                            && !key.starts_with("cmd")
                            && !key.starts_with("alt")
                            && !key.starts_with("shift")
                        {
                            this.filter_text.push_str(key);
                            this.selected_skill = None;
                            cx.notify();
                        }
                    }))
                    .child(display_text),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(Theme::text_muted())
                    .child(format!("{} {} total", total, tab_name)),
            )
            .into_any_element()
    }

    fn render_content(&self, cx: &mut Context<Self>) -> AnyElement {
        match self.active_tab {
            0 => {
                let filtered = self.filtered_skills();
                self.render_skills_list(&filtered, cx).into_any_element()
            }
            1 => {
                let filtered = self.filtered_agents();
                self.render_agents_list(&filtered).into_any_element()
            }
            2 => {
                let filtered = self.filtered_mcp_servers();
                self.render_mcp_list(&filtered).into_any_element()
            }
            _ => div().child("Unknown tab").into_any_element(),
        }
    }

    fn render_skills_list(&self, skills: &[&Skill], cx: &mut Context<Self>) -> impl IntoElement {
        if skills.is_empty() {
            return div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .h(px(200.0))
                .text_color(Theme::text_muted())
                .child("No skills found. Install skills via skills CLI or manually.");
        }

        let mut list = div().flex().flex_col().gap_2();
        for (i, skill) in skills.iter().enumerate() {
            let platform_color = match skill.platform {
                AgentPlatform::ClaudeCode => Theme::claude_badge(),
                AgentPlatform::OpenCode => Theme::opencode_badge(),
                AgentPlatform::CodexCli => Theme::codex_badge(),
                AgentPlatform::GitHubCopilot => Theme::copilot_badge(),
            };
            let is_selected = self.selected_skill == Some(i);
            let bg = if is_selected { Theme::bg_selected() } else { Theme::bg_secondary() };
            let border = if is_selected { Theme::border_active() } else { Theme::border() };

            list = list.child(
                div()
                    .id(("skill-item", i))
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_3()
                    .p_3()
                    .rounded_md()
                    .cursor_pointer()
                    .bg(bg)
                    .border_1()
                    .border_color(border)
                    .on_click(cx.listener(move |this: &mut MainView, _event: &ClickEvent, _window, cx| {
                        this.selected_skill = Some(i);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(platform_color)
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .min_w(px(60.0))
                            .justify_center()
                            .child(skill.platform.short_name().to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap_1()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child(skill.name.clone()))
                            .child(div().text_xs().text_color(Theme::text_muted()).max_w(px(300.0)).overflow_hidden().child(skill.description.clone())),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::text_muted())
                            .max_w(px(200.0))
                            .overflow_hidden()
                            .child(skill.path.to_string_lossy().to_string()),
                    ),
            );
        }
        list
    }

    fn render_agents_list(&self, agents: &[&Subagent]) -> impl IntoElement {
        if agents.is_empty() {
            return div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .h(px(200.0))
                .text_color(Theme::text_muted())
                .child("No subagents found.");
        }

        let mut list = div().flex().flex_col().gap_2();
        for agent in agents {
            let platform_color = match agent.platform {
                AgentPlatform::ClaudeCode => Theme::claude_badge(),
                AgentPlatform::OpenCode => Theme::opencode_badge(),
                AgentPlatform::CodexCli => Theme::codex_badge(),
                AgentPlatform::GitHubCopilot => Theme::copilot_badge(),
            };

            let mut tools_row = div().flex().flex_row().gap_1().flex_wrap().mt_1();
            for t in &agent.tools {
                tools_row = tools_row.child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_md()
                        .bg(Theme::bg_primary())
                        .text_xs()
                        .text_color(Theme::text_secondary())
                        .child(t.clone()),
                );
            }

            list = list.child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_3()
                    .p_3()
                    .rounded_md()
                    .bg(Theme::bg_secondary())
                    .border_1()
                    .border_color(Theme::border())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(platform_color)
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .min_w(px(60.0))
                            .justify_center()
                            .child(agent.platform.short_name().to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap_1()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child(agent.name.clone()))
                            .child(div().text_xs().text_color(Theme::text_muted()).max_w(px(300.0)).overflow_hidden().child(agent.description.clone()))
                            .child(tools_row),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(Theme::text_muted())
                            .child(format!("{:?}", agent.format)),
                    ),
            );
        }
        list
    }

    fn render_mcp_list(&self, servers: &[&MCPServer]) -> impl IntoElement {
        if servers.is_empty() {
            return div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .h(px(200.0))
                .text_color(Theme::text_muted())
                .child("No MCP servers found.");
        }

        let mut list = div().flex().flex_col().gap_2();
        for server in servers {
            let platform_color = match server.platform {
                AgentPlatform::ClaudeCode => Theme::claude_badge(),
                AgentPlatform::OpenCode => Theme::opencode_badge(),
                AgentPlatform::CodexCli => Theme::codex_badge(),
                AgentPlatform::GitHubCopilot => Theme::copilot_badge(),
            };
            let transport_str = match &server.transport {
                models::mcp::MCPTransport::Stdio { command, .. } => format!("stdio: {}", command),
                models::mcp::MCPTransport::HTTP { url, .. } => format!("http: {}", url),
            };
            let status_color = if server.enabled { Theme::success() } else { Theme::text_muted() };
            let status_text = if server.enabled { "enabled" } else { "disabled" };

            list = list.child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_3()
                    .p_3()
                    .rounded_md()
                    .bg(Theme::bg_secondary())
                    .border_1()
                    .border_color(Theme::border())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(platform_color)
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .min_w(px(60.0))
                            .justify_center()
                            .child(server.platform.short_name().to_string()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap_1()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).child(server.name.clone()))
                            .child(div().text_xs().text_color(Theme::text_muted()).child(transport_str)),
                    )
                    .child(div().text_xs().text_color(status_color).child(status_text)),
            );
        }
        list
    }

    fn render_detail_panel(&self, cx: &mut Context<Self>) -> AnyElement {
        if let Some(idx) = self.selected_skill {
            if let Some(skill) = self.filtered_skills().get(idx) {
                let mut meta_fields = div().flex().flex_col().gap_2()
                    .child(detail_field("Platform", skill.platform.name()))
                    .child(detail_field("Scope", match &skill.scope {
                        models::skill::Scope::Global => "Global",
                        models::skill::Scope::Project(_) => "Project",
                    }))
                    .child(detail_field("Path", &skill.path.to_string_lossy()));

                if let Some(ref license) = skill.license {
                    meta_fields = meta_fields.child(detail_field("License", license));
                }
                if let Some(ref compat) = skill.compatibility {
                    meta_fields = meta_fields.child(detail_field("Compatibility", compat));
                }

                return div()
                    .w(px(380.0))
                    .h_full()
                    .bg(Theme::bg_secondary())
                    .border_l_1()
                    .border_color(Theme::border())
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    // Header
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_between()
                            .items_center()
                            .child(div().text_lg().font_weight(FontWeight::SEMIBOLD).child(skill.name.clone()))
                            .child(
                                div()
                                    .id("close-detail")
                                    .px_2()
                                    .py_1()
                                    .rounded_md()
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(Theme::text_muted())
                                    .on_click(cx.listener(|this: &mut MainView, _event: &ClickEvent, _window, cx| {
                                        this.selected_skill = None;
                                        cx.notify();
                                    }))
                                    .child("Close"),
                            ),
                    )
                    // Description
                    .child(div().text_sm().text_color(Theme::text_secondary()).child(skill.description.clone()))
                    // Meta
                    .child(meta_fields)
                    // Body header
                    .child(div().text_sm().text_color(Theme::text_primary()).child("Instructions:"))
                    // Body
                    .child(
                        div()
                            .text_sm()
                            .text_color(Theme::text_muted())
                            .p_3()
                            .rounded_md()
                            .bg(Theme::bg_primary())
                            .max_h(px(300.0))
                            .id("body-scroll")
                            .overflow_scroll()
                            .child(skill.body.clone()),
                    )
                    .into_any_element();
            }
        }
        div().into_any_element()
    }
}

fn detail_field(label: &str, value: &str) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .gap_2()
        .child(div().text_xs().text_color(Theme::text_muted()).min_w(px(80.0)).child(format!("{}:", label)))
        .child(div().text_xs().text_color(Theme::text_primary()).child(value.to_string()))
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(AppState::new());
        let bounds = Bounds::centered(None, size(px(1200.0), px(750.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| MainView::new()),
        )
        .unwrap();
        cx.activate(true);
    });
}
