import os
import sys
from reportlab.lib.pagesizes import letter
from reportlab.lib import colors
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.platypus import (
    SimpleDocTemplate, Paragraph, Spacer, Table, TableStyle, PageBreak, KeepTogether, HRFlowable
)
from reportlab.pdfgen import canvas

class NumberedCanvas(canvas.Canvas):
    def __init__(self, *args, **kwargs):
        super(NumberedCanvas, self).__init__(*args, **kwargs)
        self._saved_page_states = []

    def showPage(self):
        self._saved_page_states.append(dict(self.__dict__))
        self._startPage()

    def save(self):
        num_pages = len(self._saved_page_states)
        for state in self._saved_page_states:
            self.__dict__.update(state)
            self.draw_page_number(num_pages)
            canvas.Canvas.showPage(self)
        canvas.Canvas.save(self)

    def draw_page_number(self, page_count):
        if self._pageNumber == 1:
            return  # Skip cover / first page header/footer
        self.saveState()
        self.setFont("Helvetica", 8)
        self.setFillColor(colors.HexColor("#64748b"))
        
        # Header
        self.drawString(40, 755, "NODES // AGENT-NATIVE SYSTEM DESIGN & INTERMEDIATE REPRESENTATION SPECIFICATION")
        self.setStrokeColor(colors.HexColor("#cbd5e1"))
        self.setLineWidth(0.5)
        self.line(40, 748, 572, 748)
        
        # Footer
        page_text = f"Page {self._pageNumber} of {page_count}"
        self.drawRightString(572, 35, page_text)
        self.drawString(40, 35, "CONFIDENTIAL & OPEN - FOR AI AGENT SYSTEMS & REASONING ENGINES")
        self.line(40, 47, 572, 47)
        self.restoreState()

def build_pdf(filename: str):
    doc = SimpleDocTemplate(
        filename,
        pagesize=letter,
        leftMargin=40,
        rightMargin=40,
        topMargin=50,
        bottomMargin=50
    )

    styles = getSampleStyleSheet()

    # Custom styles
    primary = colors.HexColor("#0f172a")     # Dark Slate
    brand_accent = colors.HexColor("#2563eb") # Royal Blue
    tech_cyan = colors.HexColor("#0891b2")    # Cyan
    emerald = colors.HexColor("#059669")      # Green
    code_bg = colors.HexColor("#f8fafc")
    border_color = colors.HexColor("#e2e8f0")

    title_style = ParagraphStyle(
        'CoverTitle',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=24,
        leading=28,
        textColor=primary,
        spaceAfter=8
    )

    subtitle_style = ParagraphStyle(
        'CoverSubtitle',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=11,
        leading=16,
        textColor=colors.HexColor("#475569"),
        spaceAfter=15
    )

    h1_style = ParagraphStyle(
        'Heading1',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=14,
        leading=18,
        textColor=primary,
        spaceBefore=16,
        spaceAfter=8,
        keepWithNext=True
    )

    h2_style = ParagraphStyle(
        'Heading2',
        parent=styles['Normal'],
        fontName='Helvetica-Bold',
        fontSize=11,
        leading=15,
        textColor=brand_accent,
        spaceBefore=12,
        spaceAfter=6,
        keepWithNext=True
    )

    body_style = ParagraphStyle(
        'BodyDark',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=9.5,
        leading=13.5,
        textColor=colors.HexColor("#1e293b"),
        spaceAfter=6
    )

    bullet_style = ParagraphStyle(
        'BulletText',
        parent=styles['Normal'],
        fontName='Helvetica',
        fontSize=9,
        leading=13,
        textColor=colors.HexColor("#1e293b"),
        leftIndent=14,
        firstLineIndent=-10,
        spaceAfter=4
    )

    code_style = ParagraphStyle(
        'CodeText',
        parent=styles['Normal'],
        fontName='Courier',
        fontSize=8,
        leading=11,
        textColor=colors.HexColor("#0f172a")
    )

    callout_style = ParagraphStyle(
        'CalloutText',
        parent=styles['Normal'],
        fontName='Helvetica-Oblique',
        fontSize=9,
        leading=13,
        textColor=colors.HexColor("#0369a1")
    )

    story = []

    def make_code_box(code_str: str, bg="#f8fafc", stroke="#cbd5e1"):
        p = Paragraph(code_str.replace("\n", "<br/>").replace(" ", "&nbsp;"), code_style)
        t = Table([[p]], colWidths=[532])
        t.setStyle(TableStyle([
            ('BACKGROUND', (0,0), (-1,-1), colors.HexColor(bg)),
            ('BOX', (0,0), (-1,-1), 1, colors.HexColor(stroke)),
            ('TOPPADDING', (0,0), (-1,-1), 6),
            ('BOTTOMPADDING', (0,0), (-1,-1), 6),
            ('LEFTPADDING', (0,0), (-1,-1), 8),
            ('RIGHTPADDING', (0,0), (-1,-1), 8),
        ]))
        return t

    def make_callout(text: str, title: str = "KEY PRINCIPLE FOR AI AGENTS"):
        content = f"<b>{title}:</b> {text}"
        p = Paragraph(content, callout_style)
        t = Table([[p]], colWidths=[532])
        t.setStyle(TableStyle([
            ('BACKGROUND', (0,0), (-1,-1), colors.HexColor("#f0f9ff")),
            ('BOX', (0,0), (-1,-1), 1, colors.HexColor("#bae6fd")),
            ('LINELEFT', (0,0), (0,-1), 3, colors.HexColor("#0284c7")),
            ('TOPPADDING', (0,0), (-1,-1), 6),
            ('BOTTOMPADDING', (0,0), (-1,-1), 6),
            ('LEFTPADDING', (0,0), (-1,-1), 10),
            ('RIGHTPADDING', (0,0), (-1,-1), 10),
        ]))
        return t

    # ================= COVER / TITLE =================
    story.append(Spacer(1, 10))
    story.append(Paragraph("NODES: The Agent-Native System Architecture Specification", title_style))
    story.append(Paragraph("A Complete Operational Guide and Intermediate Representation (IR) Reference for Autonomous AI Coding Agents and Human Engineers", subtitle_style))
    story.append(HRFlowable(width="100%", thickness=1.5, color=brand_accent, spaceBefore=0, spaceAfter=14))

    # ================= CHAPTER 1 =================
    story.append(Paragraph("1. Executive Summary & The Agent Alignment Problem", h1_style))
    story.append(Paragraph(
        "Modern Language Models (LLMs) and autonomous agent swarms excel at writing single functions and algorithms, but suffer from <b>architectural drift</b> when developing distributed or multi-service software. Without an explicit, persistent blueprint:",
        body_style
    ))
    story.append(Paragraph("• <b>Context Degradation:</b> Models lose track of port bindings and contracts across extensive multi-turn sessions.", bullet_style))
    story.append(Paragraph("• <b>Interface Hallucination:</b> Downstream services assume payload schemas that mismatch upstream producer interfaces.", bullet_style))
    story.append(Paragraph("• <b>Premature Code Synthesis:</b> Agents jump directly from ambiguous user prompts into writing source code files without contract agreements.", bullet_style))
    story.append(Spacer(1, 4))
    story.append(Paragraph(
        "<b>nodes</b> is a high-performance CLI tool written in Rust that provides a <b>Structured Intermediate Representation (IR)</b>. It acts as an external memory bank and contract enforcement layer. An architect agent can design the topology, define edge contracts, and assign atomic nodes to worker agents to build with zero ambiguity.",
        body_style
    ))

    story.append(Spacer(1, 4))
    story.append(make_callout(
        "Always design and validate the topology using 'nodes' before writing code. Read contracts via '--json' to ensure 100% adherence to port numbers and data schemas.",
        "MANDATORY AGENT HEURISTIC"
    ))

    # ================= CHAPTER 2 =================
    story.append(Spacer(1, 8))
    story.append(Paragraph("2. Core Architectural Primitives", h1_style))
    story.append(Paragraph(
        "The system model is composed of three atomic primitives: <b>Graphs</b>, <b>Nodes</b>, and <b>Edges</b>.",
        body_style
    ))

    # Table of Primitives
    primitives_data = [
        [Paragraph("<b>Primitive</b>", code_style), Paragraph("<b>Attributes</b>", code_style), Paragraph("<b>Semantics & Agent Guidance</b>", code_style)],
        [
            Paragraph("<b>Graph</b>", code_style),
            Paragraph("name, version, nodes, edges", code_style),
            Paragraph("An isolated architectural boundary. Created via <code>nodes --create_graph &lt;name&gt;</code>, automatically establishing a standalone shell command <code>&lt;name&gt;</code> in PATH.", body_style)
        ],
        [
            Paragraph("<b>Node</b>", code_style),
            Paragraph("name, tag, metadata, n_inputs, n_outputs", code_style),
            Paragraph("An atomic compute, storage, or interface unit. Ports are strictly 0-indexed: inputs span <code>0..(n_inputs-1)</code> and outputs span <code>0..(n_outputs-1)</code>.", body_style)
        ],
        [
            Paragraph("<b>Edge</b>", code_style),
            Paragraph("from_node, from_port, to_node, to_port, schema", code_style),
            Paragraph("A directed connection binding an output port to an input port. Carries an optional schema contract (e.g. <code>OrderEvent.json</code> or <code>Weights.safetensors</code>).", body_style)
        ]
    ]

    t_prim = Table(primitives_data, colWidths=[70, 150, 312])
    t_prim.setStyle(TableStyle([
        ('BACKGROUND', (0,0), (-1,0), colors.HexColor("#f1f5f9")),
        ('GRID', (0,0), (-1,-1), 0.5, border_color),
        ('VALIGN', (0,0), (-1,-1), 'TOP'),
        ('TOPPADDING', (0,0), (-1,-1), 5),
        ('BOTTOMPADDING', (0,0), (-1,-1), 5),
        ('LEFTPADDING', (0,0), (-1,-1), 6),
        ('RIGHTPADDING', (0,0), (-1,-1), 6),
    ]))
    story.append(t_prim)

    # ================= CHAPTER 3 =================
    story.append(Spacer(1, 8))
    story.append(Paragraph("3. Complete Command Reference for AI Tool Invocations", h1_style))
    story.append(Paragraph(
        "Agents can call these commands directly via terminal tool execution. Every command supports standard Unix options and leading dash flexibility.",
        body_style
    ))

    cmd_ref = """# 1. Graph Lifecycle
nodes --create_graph <graph_name>   # Generates standalone binary command <graph_name>
nodes graphs                         # List all registered graphs in workspace
nodes delete-graph <graph_name>      # Delete graph and its command launcher

# 2. Node Operations (Called directly using the graph command)
<graph> --add <name> --tag <tag> --metadata '<json>' --ninputs <n> --noutputs <n>
<graph> --delete <name>             # Automatically cascades and removes connected edges
<graph> inspect <name> [--json]     # Inspect node ports, incoming edges, and outgoing edges
<graph> list [--json]                # List all nodes or emit full graph IR

# 3. Connection & Wire Operations
<graph> --connect <from> -o <out_port> -i <in_port> <to> [--schema <schema_name>]
<graph> --disconnect <from> -o <out_port> -i <in_port> <to>
<graph> edges [--json]               # Table or JSON array of all active connections

# 4. Topology Visualization
<graph> draw                         # Cycle-safe ASCII stage layout with pipeline depth & metrics"""
    story.append(make_code_box(cmd_ref))

    story.append(PageBreak())

    # ================= CHAPTER 4 =================
    story.append(Paragraph("4. The Standard AI Multi-Agent Workflow", h1_style))
    story.append(Paragraph(
        "When an AI system is instructed to build a feature or architecture, it should execute the following 4-phase lifecycle:",
        body_style
    ))

    story.append(Paragraph("Phase 1: Architectural Formulation (The Architect Agent)", h2_style))
    story.append(Paragraph(
        "The Architect Agent translates unstructured requirements into discrete components with defined I/O port counts and initializes the graph:",
        body_style
    ))
    step1_code = """nodes --create_graph pdf_pipeline
pdf_pipeline --add pdf_source --tag "source" --metadata '{"format":"pdf"}' --ninputs 0 --noutputs 1
pdf_pipeline --add pdf_loader --tag "loader" --metadata '{"parser":"pdfplumber"}' --ninputs 1 --noutputs 1
pdf_pipeline --add text_chunker --tag "processor" --metadata '{"chunk_size":512}' --ninputs 1 --noutputs 1
pdf_pipeline --add vector_store --tag "sink" --metadata '{"db":"chromadb"}' --ninputs 1 --noutputs 0"""
    story.append(make_code_box(step1_code))

    story.append(Spacer(1, 4))
    story.append(Paragraph("Phase 2: Contract Binding", h2_style))
    story.append(Paragraph(
        "The Architect establishes the exact payload schemas on the directed wires before writing any code:",
        body_style
    ))
    step2_code = """pdf_pipeline --connect pdf_source -o 0 -i 0 pdf_loader --schema "RawPdf.bytes"
pdf_pipeline --connect pdf_loader -o 0 -i 0 text_chunker --schema "ExtractedPages.json"
pdf_pipeline --connect text_chunker -o 0 -i 0 vector_store --schema "EmbeddedChunks.json" """
    story.append(make_code_box(step2_code))

    story.append(Spacer(1, 4))
    story.append(Paragraph("Phase 3: Topology Verification", h2_style))
    story.append(Paragraph(
        "The Architect runs <code>draw</code> or inspects <code>list --json</code> to confirm zero unconnected ports, verify pipeline depth, and ensure no unintended dead ends.",
        body_style
    ))

    story.append(Spacer(1, 4))
    story.append(Paragraph("Phase 4: Component Implementation (Worker Subagents)", h2_style))
    story.append(Paragraph(
        "Worker agents are dispatched in parallel. Each worker inspects its single assigned node: <code>pdf_pipeline inspect text_chunker --json</code>. The output guarantees:",
        body_style
    ))
    story.append(Paragraph("• <b>Input Contract:</b> Port <code>in:0</code> receives <code>ExtractedPages.json</code>.", bullet_style))
    story.append(Paragraph("• <b>Internal Parameters:</b> Read from <code>metadata</code> (e.g. <code>chunk_size: 512</code>).", bullet_style))
    story.append(Paragraph("• <b>Output Contract:</b> Port <code>out:0</code> emits <code>EmbeddedChunks.json</code>.", bullet_style))

    # ================= CHAPTER 5 =================
    story.append(Spacer(1, 8))
    story.append(Paragraph("5. Reference Case Study: High-Throughput vLLM Inference Engine", h1_style))
    story.append(Paragraph(
        "The following diagram illustrates how complex production systems with feedback loops (continuous batching and paged KV-cache allocations) are represented in nodes:",
        body_style
    ))

    vllm_map = """========================================================================
                      SYSTEM GRAPH TOPOLOGY MAP
========================================================================
STAGE 0: INGESTION / ROOTS
  ● model_storage [storage] ──► out:0 (Weights.safetensors)
STAGE 1: INFERENCE ENGINE
  ● vllm_worker [inference_engine]
      in:0 ◄── batch_scheduler:out:0 (ContinuousBatch.json)
      in:1 ◄── model_storage:out:0   (Weights.safetensors)
      in:2 ◄── paged_kv_cache:out:0  (KVCacheTable)
      out:0 ──► token_streamer:in:0   (TokenDelta.json)
      out:1 ──► paged_kv_cache:in:0   (AllocatedBlocks)
      out:2 ──► prometheus_exporter:in:0 (TTFT_Metrics.json)
STAGE 2: MEMORY & MONITORING
  ● paged_kv_cache [cache]          ● prometheus_exporter [monitoring]
  ● token_streamer [streaming]      ● client_app [client]"""
    story.append(make_code_box(vllm_map, bg="#0f172a", stroke="#334155"))
    # Modify code style color in dark box
    
    # ================= CHAPTER 6 =================
    story.append(Spacer(1, 8))
    story.append(Paragraph("6. Strict Rules & Heuristics for AI Models", h1_style))
    story.append(Paragraph("1. <b>Never invent port numbers:</b> Port indexes always start at 0. An output with <code>n_outputs: 2</code> has ports <code>out:0</code> and <code>out:1</code> only. Specifying port 2 will trigger a bounds error.", bullet_style))
    story.append(Paragraph("2. <b>Structured Metadata:</b> Place runtime configurations, port numbers, database names, and frameworks inside <code>--metadata</code> JSON so downstream codegen prompts can ingest them directly.", bullet_style))
    story.append(Paragraph("3. <b>Machine Parsing:</b> When parsing graph state programmatically, always pass <code>--json</code>. Avoid regex scraping of ASCII tables.", bullet_style))
    story.append(Paragraph("4. <b>Cycle Safety:</b> The <code>draw</code> command is fully cycle-safe. Feel free to model request-response loops or cache cycles without fear of infinite loops.", bullet_style))

    doc.build(story, canvasmaker=NumberedCanvas)
    return filename

if __name__ == "__main__":
    out = "/home/kevin/nodes_ai_agent_guide.pdf"
    build_pdf(out)
    print(f"Generated {out}")
